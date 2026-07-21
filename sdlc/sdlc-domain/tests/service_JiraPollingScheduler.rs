use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use sdlc_domain::dto::CreateCredentialCommand::CreateCredentialCommand;
use sdlc_domain::dto::CredentialResponse::CredentialResponse;
use sdlc_domain::dto::CredentialSecretResponse::CredentialSecretResponse;
use sdlc_domain::dto::EventResponse::EventResponse;
use sdlc_domain::dto::ExecuteToolCommand::ExecuteToolCommand;
use sdlc_domain::dto::IngestEventCommand::IngestEventCommand;
use sdlc_domain::dto::IngestEventResponse::IngestEventResponse;
use sdlc_domain::dto::Project::{NewProject, Project};
use sdlc_domain::dto::RuleFiringResponse::RuleFiringResponse;
use sdlc_domain::dto::Tool::Tool;
use sdlc_domain::dto::ToolExecutionResult::ToolExecutionResult;
use sdlc_domain::dto::UpdateCredentialCommand::UpdateCredentialCommand;
use sdlc_domain::port::input::CredentialPort::CredentialPort;
use sdlc_domain::port::input::EventPort::EventPort;
use sdlc_domain::port::input::ToolExecutionPort::ToolExecutionPort;
use sdlc_domain::port::output::ProjectRepositoryPort::ProjectRepositoryPort;
use sdlc_domain::port::output::ToolRepositoryPort::ToolRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::JiraPollingScheduler::JiraPollingScheduler;

#[derive(Default)]
struct MockProjectRepository {
    projects: Mutex<Vec<Project>>,
}
#[async_trait]
impl ProjectRepositoryPort for MockProjectRepository {
    async fn save(&self, project: &Project) -> Result<(), DomainError> {
        self.projects.lock().unwrap().push(project.clone());
        Ok(())
    }
    async fn update(&self, project: &Project) -> Result<(), DomainError> {
        let mut projects = self.projects.lock().unwrap();
        if let Some(existing) = projects.iter_mut().find(|p| p.id == project.id) {
            *existing = project.clone();
        }
        Ok(())
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Project>, DomainError> {
        Ok(self.projects.lock().unwrap().iter().find(|p| p.id == id).cloned())
    }
    async fn find_by_tenant(&self, _tenant_id: Uuid) -> Result<Vec<Project>, DomainError> {
        unimplemented!()
    }
    async fn find_by_slug_and_tenant(&self, _slug: &str, _tenant_id: Uuid) -> Result<Option<Project>, DomainError> {
        unimplemented!()
    }
    async fn find_all_with_jira_tool(&self) -> Result<Vec<Project>, DomainError> {
        Ok(self.projects.lock().unwrap().iter().filter(|p| p.jira_tool_id.is_some()).cloned().collect())
    }
    async fn delete_by_id(&self, _id: Uuid) -> Result<bool, DomainError> {
        unimplemented!()
    }
}

struct MockToolRepository {
    tools: Vec<Tool>,
}
#[async_trait]
impl ToolRepositoryPort for MockToolRepository {
    async fn save(&self, _tool: &Tool) -> Result<(), DomainError> {
        unimplemented!()
    }
    async fn update(&self, _tool: &Tool) -> Result<(), DomainError> {
        unimplemented!()
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tool>, DomainError> {
        Ok(self.tools.iter().find(|t| t.id == id).cloned())
    }
    async fn find_by_tenant_including_global(&self, _tenant_id: Uuid) -> Result<Vec<Tool>, DomainError> {
        unimplemented!()
    }
    async fn delete_by_id(&self, _id: Uuid) -> Result<bool, DomainError> {
        unimplemented!()
    }
}

struct MockCredentialPort {
    secret: String,
}
#[async_trait]
impl CredentialPort for MockCredentialPort {
    async fn create_credential(&self, _command: CreateCredentialCommand) -> Result<CredentialResponse, DomainError> {
        unimplemented!()
    }
    async fn find_credential_by_id(&self, _id: Uuid) -> Result<CredentialResponse, DomainError> {
        unimplemented!()
    }
    async fn find_credentials_by_tenant(&self, _tenant_id: Uuid) -> Result<Vec<CredentialResponse>, DomainError> {
        unimplemented!()
    }
    async fn update_credential(&self, _id: Uuid, _command: UpdateCredentialCommand) -> Result<CredentialResponse, DomainError> {
        unimplemented!()
    }
    async fn delete_credential(&self, _id: Uuid) -> Result<(), DomainError> {
        unimplemented!()
    }
    async fn reveal_credential_secret(&self, id: Uuid) -> Result<CredentialSecretResponse, DomainError> {
        Ok(CredentialSecretResponse { id, provider: "jira".into(), secret: self.secret.clone() })
    }
}

#[derive(Default)]
struct MockToolExecutionPort {
    result: Mutex<Option<Result<ToolExecutionResult, String>>>,
    calls: AtomicUsize,
}
#[async_trait]
impl ToolExecutionPort for MockToolExecutionPort {
    async fn execute_tool(&self, _tool_id: Uuid, _command: ExecuteToolCommand) -> Result<ToolExecutionResult, DomainError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.result.lock().unwrap().clone().unwrap().map_err(DomainError::InternalError)
    }
}

#[derive(Default)]
struct MockEventPort {
    events: Mutex<Vec<IngestEventCommand>>,
}
#[async_trait]
impl EventPort for MockEventPort {
    async fn ingest_event(&self, command: IngestEventCommand) -> Result<IngestEventResponse, DomainError> {
        self.events.lock().unwrap().push(command);
        Ok(IngestEventResponse {
            event: EventResponse { id: Uuid::new_v4(), tenant_id: Uuid::new_v4(), event_type: String::new(), payload: "{}".into(), received_at: Utc::now() },
            firings: vec![],
        })
    }
    async fn find_events_by_tenant(&self, _tenant_id: Uuid) -> Result<Vec<EventResponse>, DomainError> {
        unimplemented!()
    }
    async fn find_firings_by_event(&self, _event_id: Uuid) -> Result<Vec<RuleFiringResponse>, DomainError> {
        unimplemented!()
    }
}

fn sample_tool(id: Uuid, config: serde_json::Value, is_enabled: bool) -> Tool {
    let now = Utc::now();
    Tool { id, tenant_id: None, name: "Jira".into(), tool_type: "jira".into(), description: None, config: Some(config.to_string()), is_enabled, created_at: now, updated_at: now, created_by: None, updated_by: None }
}

fn sample_project(tenant_id: Uuid, jira_tool_id: Option<Uuid>) -> Project {
    Project::new(NewProject {
        tenant_id,
        name: "Test".into(),
        slug: format!("test-{}", Uuid::new_v4()),
        github_tool_id: Uuid::new_v4(),
        default_branch: "main".into(),
        jira_tool_id,
        build_command: None,
        test_command: None,
        coding_standards: None,
        workflow_config: None,
    })
}

fn valid_jira_config(credential_id: Uuid) -> serde_json::Value {
    serde_json::json!({ "project_key": "PROJ", "email": "bot@example.com", "credential_id": credential_id })
}

#[tokio::test]
async fn poll_once_ingests_new_issues_and_updates_cursor() {
    let jira_tool_id = Uuid::new_v4();
    let credential_id = Uuid::new_v4();
    let project = sample_project(Uuid::new_v4(), Some(jira_tool_id));
    let project_id = project.id;

    let project_repo = Arc::new(MockProjectRepository::default());
    project_repo.projects.lock().unwrap().push(project);
    let tool_repo = Arc::new(MockToolRepository { tools: vec![sample_tool(jira_tool_id, valid_jira_config(credential_id), true)] });

    let search_body = serde_json::json!({
        "issues": [
            { "key": "PROJ-1", "fields": { "summary": "Fix bug", "issuetype": {"name": "Bug"} } },
            { "key": "PROJ-2", "fields": { "summary": "Add feature", "issuetype": {"name": "Story"} } },
        ]
    }).to_string();
    let tool_execution = Arc::new(MockToolExecutionPort::default());
    *tool_execution.result.lock().unwrap() = Some(Ok(ToolExecutionResult::ok(search_body, 10)));
    let event_port = Arc::new(MockEventPort::default());

    let scheduler = JiraPollingScheduler::new(project_repo.clone(), tool_repo, Arc::new(MockCredentialPort { secret: "token".into() }), tool_execution.clone(), event_port.clone());
    scheduler.poll_once().await.unwrap();

    assert_eq!(tool_execution.calls.load(Ordering::SeqCst), 1);
    let events = event_port.events.lock().unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].event_type, "jira.ticket.updated");
    assert_eq!(events[0].payload["ticket_key"], "PROJ-1");
    assert_eq!(events[1].payload["ticket_key"], "PROJ-2");

    let updated = project_repo.projects.lock().unwrap().iter().find(|p| p.id == project_id).unwrap().clone();
    assert!(updated.jira_last_synced_at.is_some());
}

#[tokio::test]
async fn poll_once_skips_projects_with_a_configured_webhook() {
    let jira_tool_id = Uuid::new_v4();
    let mut config = valid_jira_config(Uuid::new_v4());
    config["webhook_secret"] = serde_json::json!("shh");
    let project = sample_project(Uuid::new_v4(), Some(jira_tool_id));

    let project_repo = Arc::new(MockProjectRepository::default());
    project_repo.projects.lock().unwrap().push(project);
    let tool_repo = Arc::new(MockToolRepository { tools: vec![sample_tool(jira_tool_id, config, true)] });
    let tool_execution = Arc::new(MockToolExecutionPort::default());
    let event_port = Arc::new(MockEventPort::default());

    let scheduler = JiraPollingScheduler::new(project_repo, tool_repo, Arc::new(MockCredentialPort { secret: "token".into() }), tool_execution.clone(), event_port.clone());
    scheduler.poll_once().await.unwrap();

    assert_eq!(tool_execution.calls.load(Ordering::SeqCst), 0);
    assert_eq!(event_port.events.lock().unwrap().len(), 0);
}

#[tokio::test]
async fn poll_once_skips_a_disabled_jira_tool() {
    let jira_tool_id = Uuid::new_v4();
    let project = sample_project(Uuid::new_v4(), Some(jira_tool_id));

    let project_repo = Arc::new(MockProjectRepository::default());
    project_repo.projects.lock().unwrap().push(project);
    let tool_repo = Arc::new(MockToolRepository { tools: vec![sample_tool(jira_tool_id, valid_jira_config(Uuid::new_v4()), false)] });
    let tool_execution = Arc::new(MockToolExecutionPort::default());
    let event_port = Arc::new(MockEventPort::default());

    let scheduler = JiraPollingScheduler::new(project_repo, tool_repo, Arc::new(MockCredentialPort { secret: "token".into() }), tool_execution.clone(), event_port.clone());
    scheduler.poll_once().await.unwrap();

    assert_eq!(tool_execution.calls.load(Ordering::SeqCst), 0);
    assert_eq!(event_port.events.lock().unwrap().len(), 0);
}

#[tokio::test]
async fn poll_once_continues_past_a_project_whose_config_is_incomplete() {
    let broken_tool_id = Uuid::new_v4();
    let broken_project = sample_project(Uuid::new_v4(), Some(broken_tool_id));

    let working_tool_id = Uuid::new_v4();
    let credential_id = Uuid::new_v4();
    let working_project = sample_project(Uuid::new_v4(), Some(working_tool_id));

    let project_repo = Arc::new(MockProjectRepository::default());
    project_repo.projects.lock().unwrap().push(broken_project);
    project_repo.projects.lock().unwrap().push(working_project);

    let tool_repo = Arc::new(MockToolRepository {
        tools: vec![
            sample_tool(broken_tool_id, serde_json::json!({ "email": "bot@example.com" }), true), // missing project_key
            sample_tool(working_tool_id, valid_jira_config(credential_id), true),
        ],
    });

    let search_body = serde_json::json!({ "issues": [{ "key": "PROJ-1", "fields": { "summary": "Fix bug" } }] }).to_string();
    let tool_execution = Arc::new(MockToolExecutionPort::default());
    *tool_execution.result.lock().unwrap() = Some(Ok(ToolExecutionResult::ok(search_body, 10)));
    let event_port = Arc::new(MockEventPort::default());

    let scheduler = JiraPollingScheduler::new(project_repo, tool_repo, Arc::new(MockCredentialPort { secret: "token".into() }), tool_execution.clone(), event_port.clone());
    let result = scheduler.poll_once().await;

    assert!(result.is_ok());
    assert_eq!(tool_execution.calls.load(Ordering::SeqCst), 1);
    assert_eq!(event_port.events.lock().unwrap().len(), 1);
}
