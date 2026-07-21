use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use tracing::{error, info};
use uuid::Uuid;
use sdlc_domain::dto::CreateCredentialCommand::CreateCredentialCommand;
use sdlc_domain::dto::CredentialResponse::CredentialResponse;
use sdlc_domain::dto::CredentialSecretResponse::CredentialSecretResponse;
use sdlc_domain::dto::KnowledgeSnippet::KnowledgeSnippet;
use sdlc_domain::dto::ProjectContext::ProjectContext;
use sdlc_domain::dto::SdlcRun::{NewSdlcRun, SdlcRun};
use sdlc_domain::dto::SdlcRunResponse::SdlcRunResponse;
use sdlc_domain::dto::SdlcRunStatus::SdlcRunStatus;
use sdlc_domain::dto::SdlcStep::SdlcStep;
use sdlc_domain::dto::SdlcStepExecution::{NewSdlcStepExecution, SdlcStepExecution};
use sdlc_domain::dto::SdlcStepExecutionResponse::SdlcStepExecutionResponse;
use sdlc_domain::dto::StartSdlcRunCommand::StartSdlcRunCommand;
use sdlc_domain::dto::StepExecutionStatus::StepExecutionStatus;
use sdlc_domain::dto::Tool::Tool;
use sdlc_domain::dto::UpdateCredentialCommand::UpdateCredentialCommand;
use sdlc_domain::port::input::CredentialPort::CredentialPort;
use sdlc_domain::port::input::SdlcRunPort::SdlcRunPort;
use sdlc_domain::port::output::AgentsClientPort::AgentsClientPort;
use sdlc_domain::port::output::KnowledgeClientPort::KnowledgeClientPort;
use sdlc_domain::port::output::PlatformClientPort::PlatformClientPort;
use sdlc_domain::port::output::SdlcCheckpointRepositoryPort::SdlcCheckpointRepositoryPort;
use sdlc_domain::port::output::SdlcRunRepositoryPort::SdlcRunRepositoryPort;
use sdlc_domain::port::output::SdlcStepExecutionRepositoryPort::SdlcStepExecutionRepositoryPort;
use sdlc_domain::port::output::SdlcToolsClientPort::SdlcToolsClientPort;
use sdlc_domain::port::output::ToolRepositoryPort::ToolRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::SdlcOrchestratorService::SdlcOrchestratorService;

use std::sync::Mutex;

#[derive(Default)]
struct MockRunRepository {
    runs: Mutex<Vec<SdlcRun>>,
}

#[async_trait]
impl SdlcRunRepositoryPort for MockRunRepository {
    async fn save(&self, run: &SdlcRun) -> Result<(), DomainError> {
        self.runs.lock().unwrap().push(run.clone());
        Ok(())
    }
    async fn update(&self, run: &SdlcRun) -> Result<(), DomainError> {
        let mut runs = self.runs.lock().unwrap();
        if let Some(existing) = runs.iter_mut().find(|r| r.id == run.id) {
            *existing = run.clone();
        }
        Ok(())
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SdlcRun>, DomainError> {
        Ok(self.runs.lock().unwrap().iter().find(|r| r.id == id).cloned())
    }
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<SdlcRun>, DomainError> {
        Ok(self.runs.lock().unwrap().iter().filter(|r| r.tenant_id == tenant_id).cloned().collect())
    }
}

#[derive(Default)]
struct MockStepRepository {
    rows: Mutex<Vec<SdlcStepExecution>>,
}

#[async_trait]
impl SdlcStepExecutionRepositoryPort for MockStepRepository {
    async fn save(&self, step: &SdlcStepExecution) -> Result<(), DomainError> {
        self.rows.lock().unwrap().push(step.clone());
        Ok(())
    }
    async fn update(&self, step: &SdlcStepExecution) -> Result<(), DomainError> {
        let mut rows = self.rows.lock().unwrap();
        if let Some(existing) = rows.iter_mut().find(|r| r.id == step.id) {
            *existing = step.clone();
        }
        Ok(())
    }
    async fn find_by_run_id(&self, run_id: Uuid) -> Result<Vec<SdlcStepExecution>, DomainError> {
        Ok(self.rows.lock().unwrap().iter().filter(|r| r.run_id == run_id).cloned().collect())
    }
}

/// Mirrors the real adapter's atomicity contract (step + run land together)
/// by writing through the same two in-memory stores under one lock.
struct MockCheckpointRepository {
    run_repository: Arc<MockRunRepository>,
    step_repository: Arc<MockStepRepository>,
}

#[async_trait]
impl SdlcCheckpointRepositoryPort for MockCheckpointRepository {
    async fn save_checkpoint(&self, step: &SdlcStepExecution, run: &SdlcRun) -> Result<(), DomainError> {
        self.step_repository.update(step).await?;
        self.run_repository.update(run).await?;
        Ok(())
    }
}

struct MockPlatformClient;
#[async_trait]
impl PlatformClientPort for MockPlatformClient {
    async fn get_project(&self, _project_id: Uuid) -> Result<ProjectContext, DomainError> {
        Ok(ProjectContext {
            repository_url: "https://example.com/repo.git".into(),
            default_branch: "main".into(),
            jira_project_key: Some("PROJ".into()),
            build_command: Some("build".into()),
            test_command: Some("test".into()),
        })
    }
}

struct MockKnowledgeClient;
#[async_trait]
impl KnowledgeClientPort for MockKnowledgeClient {
    async fn search(&self, _tenant_id: Uuid, _query: String, _limit: u32) -> Result<Vec<KnowledgeSnippet>, DomainError> {
        Ok(vec![KnowledgeSnippet { title: "ADR-1".into(), content: "Use hexagonal architecture".into(), score: 0.9 }])
    }
}

struct MockAgentsClient {
    /// Overrides what's returned for the Developer prompt specifically (the one
    /// containing "Respond with STRICT JSON ONLY"); `None` uses a valid canned
    /// file-edit response. Every other agent call just gets generic text back.
    developer_output: Option<String>,
}

impl Default for MockAgentsClient {
    fn default() -> Self {
        Self { developer_output: None }
    }
}

#[async_trait]
impl AgentsClientPort for MockAgentsClient {
    async fn execute_agent(&self, _agent_id: Uuid, _tenant_id: Uuid, input: String) -> Result<String, DomainError> {
        if input.contains("Respond with STRICT JSON ONLY") {
            Ok(self.developer_output.clone().unwrap_or_else(|| {
                r#"{"summary": "Added rate limiting middleware", "files": [{"path": "src/lib.rs", "content": "// rate limiting"}]}"#.into()
            }))
        } else {
            Ok(format!("agent-output: {}", input))
        }
    }
}

struct MockToolsClient {
    build_calls: Mutex<u32>,
    build_fail_times: u32,
    test_should_succeed: bool,
    github_should_succeed: bool,
    jira_should_succeed: bool,
}

impl Default for MockToolsClient {
    fn default() -> Self {
        Self { build_calls: Mutex::new(0), build_fail_times: 0, test_should_succeed: true, github_should_succeed: true, jira_should_succeed: true }
    }
}

use sdlc_domain::dto::ToolRunResult::ToolRunResult;

#[async_trait]
impl SdlcToolsClientPort for MockToolsClient {
    async fn execute_tool(&self, _tool_id: Uuid, action: String, parameters: HashMap<String, String>, _working_directory: Option<String>) -> Result<ToolRunResult, DomainError> {
        // The orchestrator uses the generic "run" action (with the project's
        // command as "args") whenever a project-specific build/test command is
        // configured — sample_command's MockPlatformClient always sets one, so
        // "run" is what actually gets exercised here; distinguish build vs. test
        // by which command was passed.
        let effective_action = if action == "run" {
            match parameters.get("args").map(String::as_str) {
                Some("build") => "build",
                Some("test") => "test",
                _ => "run",
            }
        } else {
            action.as_str()
        };

        match effective_action {
            "checkout" | "commit" | "push" => Ok(ToolRunResult { success: true, output: "ok".into(), error: None }),
            "build" => {
                let mut calls = self.build_calls.lock().unwrap();
                *calls += 1;
                if *calls <= self.build_fail_times {
                    Ok(ToolRunResult { success: false, output: String::new(), error: Some("compile error".into()) })
                } else {
                    Ok(ToolRunResult { success: true, output: "build ok".into(), error: None })
                }
            }
            "test" => Ok(ToolRunResult {
                success: self.test_should_succeed,
                output: "tests".into(),
                error: if self.test_should_succeed { None } else { Some("test failure".into()) },
            }),
            "get_issue" => Ok(ToolRunResult {
                success: self.jira_should_succeed,
                output: "Ticket summary text".into(),
                error: if self.jira_should_succeed { None } else { Some("jira error".into()) },
            }),
            "add_comment" => Ok(ToolRunResult { success: self.jira_should_succeed, output: "commented".into(), error: None }),
            "write" => Ok(ToolRunResult { success: true, output: "written".into(), error: None }),
            "create_pull_request" => Ok(ToolRunResult {
                success: self.github_should_succeed,
                output: r#"{"html_url":"https://github.com/org/repo/pull/1"}"#.into(),
                error: None,
            }),
            other => Err(DomainError::InternalError(format!("unexpected tool action '{}'", other))),
        }
    }
}

/// Always resolves any tool id to a Jira tool with valid config — the tests
/// pass fresh random ids for github/jira tools, so lookups can't be keyed to a
/// specific id.
struct MockToolRepository;
#[async_trait]
impl ToolRepositoryPort for MockToolRepository {
    async fn save(&self, _tool: &Tool) -> Result<(), DomainError> { unimplemented!() }
    async fn update(&self, _tool: &Tool) -> Result<(), DomainError> { unimplemented!() }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tool>, DomainError> {
        let now = Utc::now();
        Ok(Some(Tool {
            id,
            tenant_id: None,
            name: "Jira".into(),
            tool_type: "jira".into(),
            description: None,
            config: Some(serde_json::json!({ "email": "bot@example.com", "credential_id": Uuid::new_v4() }).to_string()),
            is_enabled: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }))
    }
    async fn find_by_tenant_including_global(&self, _tenant_id: Uuid) -> Result<Vec<Tool>, DomainError> { unimplemented!() }
    async fn delete_by_id(&self, _id: Uuid) -> Result<bool, DomainError> { unimplemented!() }
}

struct MockCredentialPort;
#[async_trait]
impl CredentialPort for MockCredentialPort {
    async fn create_credential(&self, _command: CreateCredentialCommand) -> Result<CredentialResponse, DomainError> { unimplemented!() }
    async fn find_credential_by_id(&self, _id: Uuid) -> Result<CredentialResponse, DomainError> { unimplemented!() }
    async fn find_credentials_by_tenant(&self, _tenant_id: Uuid) -> Result<Vec<CredentialResponse>, DomainError> { unimplemented!() }
    async fn update_credential(&self, _id: Uuid, _command: UpdateCredentialCommand) -> Result<CredentialResponse, DomainError> { unimplemented!() }
    async fn delete_credential(&self, _id: Uuid) -> Result<(), DomainError> { unimplemented!() }
    async fn reveal_credential_secret(&self, id: Uuid) -> Result<CredentialSecretResponse, DomainError> {
        Ok(CredentialSecretResponse { id, provider: "jira".into(), secret: "token".into() })
    }
}

fn sample_command(github_tool_id: Option<Uuid>, jira_tool_id: Option<Uuid>) -> StartSdlcRunCommand {
    StartSdlcRunCommand {
        tenant_id: Uuid::new_v4(),
        project_id: Uuid::new_v4(),
        ticket_key: "PROJ-123".into(),
        ticket_summary: Some("Add rate limiting to the API gateway".into()),
        planner_agent_id: Uuid::new_v4(),
        architect_agent_id: Uuid::new_v4(),
        developer_agent_id: Uuid::new_v4(),
        reviewer_agent_id: Uuid::new_v4(),
        documentation_agent_id: Uuid::new_v4(),
        git_tool_id: Uuid::new_v4(),
        build_tool_id: Uuid::new_v4(),
        filesystem_tool_id: Uuid::new_v4(),
        github_tool_id,
        jira_tool_id,
    }
}

fn fixture(tools_client: MockToolsClient) -> (SdlcOrchestratorService, Arc<MockStepRepository>) {
    fixture_with_developer_output(tools_client, None)
}

fn fixture_with_developer_output(tools_client: MockToolsClient, developer_output: Option<&str>) -> (SdlcOrchestratorService, Arc<MockStepRepository>) {
    let run_repo = Arc::new(MockRunRepository::default());
    let step_repo = Arc::new(MockStepRepository::default());
    let checkpoint_repo = Arc::new(MockCheckpointRepository {
        run_repository: run_repo.clone(),
        step_repository: step_repo.clone(),
    });
    let service = SdlcOrchestratorService::new(
        run_repo,
        step_repo.clone(),
        checkpoint_repo,
        Arc::new(MockPlatformClient),
        Arc::new(MockKnowledgeClient),
        Arc::new(MockAgentsClient { developer_output: developer_output.map(String::from) }),
        Arc::new(tools_client),
        Arc::new(MockToolRepository),
        Arc::new(MockCredentialPort),
    );
    (service, step_repo)
}

#[tokio::test]
async fn start_run_completes_the_happy_path_without_optional_steps() {
    let (service, step_repo) = fixture(MockToolsClient::default());
    let response = service.start_run(sample_command(None, None)).await.unwrap();
    assert_eq!(response.status, "completed");
    assert!(response.branch_name.is_some());
    assert!(response.pull_request_url.is_none());

    let steps = step_repo.rows.lock().unwrap();
    assert!(!steps.iter().any(|s| s.step == SdlcStep::FetchTicket));
    assert!(!steps.iter().any(|s| s.step == SdlcStep::CreatePullRequest));
    assert!(!steps.iter().any(|s| s.step == SdlcStep::UpdateJira));
    assert!(steps.iter().any(|s| s.step == SdlcStep::ReviewerAgent && s.status == StepExecutionStatus::Succeeded));
}

#[tokio::test]
async fn start_run_runs_optional_steps_when_github_and_jira_are_configured() {
    let (service, step_repo) = fixture(MockToolsClient::default());
    let response = service.start_run(sample_command(Some(Uuid::new_v4()), Some(Uuid::new_v4()))).await.unwrap();
    assert_eq!(response.status, "completed");
    assert_eq!(response.pull_request_url.unwrap(), "https://github.com/org/repo/pull/1");

    let steps = step_repo.rows.lock().unwrap();
    assert!(steps.iter().any(|s| s.step == SdlcStep::FetchTicket));
    assert!(steps.iter().any(|s| s.step == SdlcStep::UpdateJira));
}

#[tokio::test]
async fn start_run_retries_compile_until_it_succeeds() {
    let tools = MockToolsClient { build_fail_times: 2, ..MockToolsClient::default() };
    let (service, step_repo) = fixture(tools);
    let response = service.start_run(sample_command(None, None)).await.unwrap();
    assert_eq!(response.status, "completed");

    let steps = step_repo.rows.lock().unwrap();
    let compile_attempts: Vec<_> = steps.iter().filter(|s| s.step == SdlcStep::Compile).collect();
    assert_eq!(compile_attempts.len(), 3);
    assert!(compile_attempts.iter().any(|s| s.status == StepExecutionStatus::Succeeded));
}

#[tokio::test]
async fn start_run_fails_when_compile_never_succeeds() {
    let tools = MockToolsClient { build_fail_times: 100, ..MockToolsClient::default() };
    let (service, _step_repo) = fixture(tools);
    let response = service.start_run(sample_command(None, None)).await.unwrap();
    assert_eq!(response.status, "failed");
    assert!(response.error.unwrap().contains("compile"));
}

#[tokio::test]
async fn start_run_fails_validation_when_ticket_key_is_empty() {
    let (service, _step_repo) = fixture(MockToolsClient::default());
    let mut command = sample_command(None, None);
    command.ticket_key = "".into();
    let result = service.start_run(command).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn start_run_writes_developer_agent_file_edits() {
    let (service, step_repo) = fixture(MockToolsClient::default());
    let response = service.start_run(sample_command(None, None)).await.unwrap();
    assert_eq!(response.status, "completed");

    let steps = step_repo.rows.lock().unwrap();
    let developer_step = steps.iter().find(|s| s.step == SdlcStep::DeveloperAgent && s.status == StepExecutionStatus::Succeeded).unwrap();
    let output = developer_step.output.as_ref().unwrap();
    assert_eq!(output["files_written"], serde_json::json!(["src/lib.rs"]));
}

#[tokio::test]
async fn start_run_fails_when_developer_agent_output_is_not_valid_file_edit_json() {
    let (service, step_repo) = fixture_with_developer_output(MockToolsClient::default(), Some("I'll get right on that!"));
    let response = service.start_run(sample_command(None, None)).await.unwrap();
    assert_eq!(response.status, "failed");

    let steps = step_repo.rows.lock().unwrap();
    assert!(steps.iter().any(|s| s.step == SdlcStep::DeveloperAgent && s.status == StepExecutionStatus::Failed));
}
