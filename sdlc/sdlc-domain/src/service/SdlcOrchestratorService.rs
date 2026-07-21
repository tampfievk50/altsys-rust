use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{error, info};
use uuid::Uuid;

use crate::dto::KnowledgeSnippet::KnowledgeSnippet;
use crate::dto::ProjectContext::ProjectContext;
use crate::dto::SdlcRun::{NewSdlcRun, SdlcRun};
use crate::dto::SdlcRunResponse::SdlcRunResponse;
use crate::dto::SdlcRunStatus::SdlcRunStatus;
use crate::dto::SdlcStep::SdlcStep;
use crate::dto::SdlcStepExecution::{NewSdlcStepExecution, SdlcStepExecution};
use crate::dto::SdlcStepExecutionResponse::SdlcStepExecutionResponse;
use crate::dto::StartSdlcRunCommand::StartSdlcRunCommand;
use crate::dto::StepExecutionStatus::StepExecutionStatus;
use crate::port::input::CredentialPort::CredentialPort;
use crate::port::input::SdlcRunPort::SdlcRunPort;
use crate::port::output::AgentsClientPort::AgentsClientPort;
use crate::port::output::KnowledgeClientPort::KnowledgeClientPort;
use crate::port::output::PlatformClientPort::PlatformClientPort;
use crate::port::output::SdlcCheckpointRepositoryPort::SdlcCheckpointRepositoryPort;
use crate::port::output::SdlcRunRepositoryPort::SdlcRunRepositoryPort;
use crate::port::output::SdlcStepExecutionRepositoryPort::SdlcStepExecutionRepositoryPort;
use crate::port::output::SdlcToolsClientPort::SdlcToolsClientPort;
use crate::port::output::ToolRepositoryPort::ToolRepositoryPort;
use crate::r#enum::DomainError::DomainError;

/// Bounds the "Fix Build Errors (loop)" / "Fix Test Failures (loop)" cycles from
/// the roadmap diagram.
const MAX_FIX_ATTEMPTS: i32 = 3;

pub struct SdlcOrchestratorService {
    run_repository: Arc<dyn SdlcRunRepositoryPort>,
    step_repository: Arc<dyn SdlcStepExecutionRepositoryPort>,
    checkpoint_repository: Arc<dyn SdlcCheckpointRepositoryPort>,
    platform_client: Arc<dyn PlatformClientPort>,
    knowledge_client: Arc<dyn KnowledgeClientPort>,
    agents_client: Arc<dyn AgentsClientPort>,
    tools_client: Arc<dyn SdlcToolsClientPort>,
    tool_repository: Arc<dyn ToolRepositoryPort>,
    credential_port: Arc<dyn CredentialPort>,
}

impl SdlcOrchestratorService {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        run_repository: Arc<dyn SdlcRunRepositoryPort>,
        step_repository: Arc<dyn SdlcStepExecutionRepositoryPort>,
        checkpoint_repository: Arc<dyn SdlcCheckpointRepositoryPort>,
        platform_client: Arc<dyn PlatformClientPort>,
        knowledge_client: Arc<dyn KnowledgeClientPort>,
        agents_client: Arc<dyn AgentsClientPort>,
        tools_client: Arc<dyn SdlcToolsClientPort>,
        tool_repository: Arc<dyn ToolRepositoryPort>,
        credential_port: Arc<dyn CredentialPort>,
    ) -> Self {
        Self { run_repository, step_repository, checkpoint_repository, platform_client, knowledge_client, agents_client, tools_client, tool_repository, credential_port }
    }

    /// Resolves the `(email, api_token)` pair `JiraExecutor` needs for basic auth
    /// from the Jira Tool's `config` (`email`, `credential_id`) — the same
    /// resolution `JiraPollingScheduler` does for polling, needed here too since
    /// `ToolExecutionService` doesn't auto-inject credentials into parameters.
    async fn resolve_jira_credentials(&self, jira_tool_id: Uuid) -> Result<(String, String), DomainError> {
        let tool = self.tool_repository.find_by_id(jira_tool_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Jira tool not found: {}", jira_tool_id)))?;
        let config: serde_json::Value = tool.config.as_deref()
            .and_then(|c| serde_json::from_str(c).ok())
            .unwrap_or_default();
        let email = config.get("email").and_then(|v| v.as_str())
            .ok_or_else(|| DomainError::ValidationError("Jira tool config missing 'email'".into()))?
            .to_string();
        let credential_id: Uuid = config.get("credential_id").and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| DomainError::ValidationError("Jira tool config missing 'credential_id'".into()))?;
        let api_token = self.credential_port.reveal_credential_secret(credential_id).await?.secret;
        Ok((email, api_token))
    }

    /// Resolves the personal access token `GitHubExecutor` needs from the
    /// GitHub Tool's config (`credential_id`) — same rationale as
    /// `resolve_jira_credentials`.
    async fn resolve_github_token(&self, github_tool_id: Uuid) -> Result<String, DomainError> {
        let tool = self.tool_repository.find_by_id(github_tool_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("GitHub tool not found: {}", github_tool_id)))?;
        let config: serde_json::Value = tool.config.as_deref()
            .and_then(|c| serde_json::from_str(c).ok())
            .unwrap_or_default();
        let credential_id: Uuid = config.get("credential_id").and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| DomainError::ValidationError("GitHub tool config missing 'credential_id'".into()))?;
        Ok(self.credential_port.reveal_credential_secret(credential_id).await?.secret)
    }

    /// LLM output sometimes wraps the requested JSON in prose or a code fence;
    /// take the outermost `{...}` span rather than requiring an exact match.
    fn extract_json_object(text: &str) -> Option<&str> {
        let start = text.find('{')?;
        let end = text.rfind('}')?;
        if end < start {
            return None;
        }
        Some(&text[start..=end])
    }

    /// Parses the Developer agent's `{"files": [{"path", "content"}, ...]}`
    /// response. Returns `None` if the JSON is missing, malformed, or has no
    /// usable file entries, so the caller can fail the step with a clear error
    /// instead of silently doing nothing.
    fn parse_file_edits(text: &str) -> Option<Vec<(String, String)>> {
        let json_slice = Self::extract_json_object(text)?;
        let parsed: serde_json::Value = serde_json::from_str(json_slice).ok()?;
        let files = parsed.get("files")?.as_array()?;
        let edits: Vec<(String, String)> = files.iter()
            .filter_map(|f| {
                let path = f.get("path")?.as_str()?.to_string();
                let content = f.get("content")?.as_str()?.to_string();
                Some((path, content))
            })
            .collect();
        if edits.is_empty() { None } else { Some(edits) }
    }

    /// Writes each edit through the Filesystem tool (`working_directory` must
    /// match the git checkout `git_tool_id` created the branch in). Returns the
    /// paths written on success; the first failure aborts the whole step.
    async fn apply_file_edits(&self, filesystem_tool_id: Uuid, edits: &[(String, String)]) -> Result<Vec<String>, DomainError> {
        let mut written = Vec::with_capacity(edits.len());
        for (path, content) in edits {
            let mut params = HashMap::new();
            params.insert("path".to_string(), path.clone());
            params.insert("content".to_string(), content.clone());
            let result = self.tools_client.execute_tool(filesystem_tool_id, "write".into(), params, None).await?;
            if !result.success {
                return Err(DomainError::InternalError(format!("Failed to write '{}': {}", path, result.error.unwrap_or_default())));
            }
            written.push(path.clone());
        }
        Ok(written)
    }

    fn to_response(run: &SdlcRun) -> SdlcRunResponse {
        SdlcRunResponse {
            id: run.id,
            tenant_id: run.tenant_id,
            project_id: run.project_id,
            ticket_key: run.ticket_key.clone(),
            status: run.status.to_string(),
            current_step: run.current_step.clone(),
            branch_name: run.branch_name.clone(),
            pull_request_url: run.pull_request_url.clone(),
            context: run.context.to_string(),
            error: run.error.clone(),
            started_at: run.started_at,
            completed_at: run.completed_at,
            created_at: run.created_at,
            updated_at: run.updated_at,
        }
    }

    fn to_step_response(step: &SdlcStepExecution) -> SdlcStepExecutionResponse {
        SdlcStepExecutionResponse {
            id: step.id,
            run_id: step.run_id,
            step: step.step.to_string(),
            attempt: step.attempt,
            status: step.status.to_string(),
            output: step.output.as_ref().map(|v| v.to_string()),
            error: step.error.clone(),
            started_at: step.started_at,
            completed_at: step.completed_at,
        }
    }

    fn knowledge_to_text(snippets: &[KnowledgeSnippet]) -> String {
        if snippets.is_empty() {
            return "(no relevant organizational knowledge found)".to_string();
        }
        snippets.iter()
            .map(|s| format!("- {} (score {:.2}): {}", s.title, s.score, s.content))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn extract_pr_url(pr_output: &serde_json::Value) -> Option<String> {
        let output = pr_output.get("output")?.as_str()?;
        let parsed: serde_json::Value = serde_json::from_str(output).ok()?;
        parsed.get("html_url").and_then(|v| v.as_str()).map(str::to_string)
    }

    async fn start_step(&self, run_id: Uuid, step: SdlcStep, attempt: i32) -> Result<SdlcStepExecution, DomainError> {
        let step_execution = SdlcStepExecution::new(NewSdlcStepExecution { run_id, step, attempt });
        self.step_repository.save(&step_execution).await?;
        Ok(step_execution)
    }

    /// Runs one step end to end: checkpoints it, merges its output into the run's
    /// context, and persists the run's progress. `work` is already-awaited so this
    /// carries no closure-capture complexity.
    async fn run_step(&self, run: &mut SdlcRun, step: SdlcStep, work: Result<serde_json::Value, DomainError>) -> Result<serde_json::Value, DomainError> {
        run.current_step = Some(step.to_string());
        let mut step_execution = self.start_step(run.id, step, 1).await?;

        let result = match work {
            Ok(output) => {
                step_execution.status = StepExecutionStatus::Succeeded;
                step_execution.output = Some(output.clone());
                Ok(output)
            }
            Err(e) => {
                step_execution.status = StepExecutionStatus::Failed;
                step_execution.error = Some(e.to_string());
                Err(e)
            }
        };
        step_execution.completed_at = Some(Utc::now());
        step_execution.updated_at = Utc::now();

        if let Ok(output) = &result {
            run.merge_context(&step.to_string(), output);
        }
        run.updated_at = Utc::now();
        // The step's outcome and the run's advanced context/current_step are two
        // rows that must land together — a partial write here would leave the run
        // and its step history disagreeing about what actually happened.
        self.checkpoint_repository.save_checkpoint(&step_execution, run).await?;
        result
    }

    /// Runs `build_tool_id`'s `action` (`"build"` or `"test"`), asking the developer
    /// agent for a fix suggestion (recorded on the checkpoint, not auto-applied)
    /// after each failed attempt — the "Compile / Fix Build Errors (loop)" and
    /// "Run Tests / Fix Test Failures (loop)" pairs in the roadmap diagram.
    #[allow(clippy::too_many_arguments)]
    async fn run_build_tool_with_fix_loop(
        &self,
        run: &mut SdlcRun,
        step: SdlcStep,
        default_action: &str,
        context_key: &str,
        build_tool_id: Uuid,
        developer_agent_id: Uuid,
        tenant_id: Uuid,
        command: Option<&str>,
    ) -> Result<(), DomainError> {
        for attempt in 1..=MAX_FIX_ATTEMPTS {
            run.current_step = Some(step.to_string());
            let mut step_execution = self.start_step(run.id, step, attempt).await?;

            // A project-specific command (e.g. "cargo build --release") runs verbatim
            // via the tool's generic "run" action; otherwise fall back to the tool's
            // own semantic default action (plain `cargo build` / `cargo test`, etc.).
            // Passing `command` as extra args *alongside* `default_action` would double
            // up (`cargo build build`), so the two are mutually exclusive.
            let (action, params) = match command {
                Some(cmd) => {
                    let mut params = HashMap::new();
                    params.insert("args".to_string(), cmd.to_string());
                    ("run".to_string(), params)
                }
                None => (default_action.to_string(), HashMap::new()),
            };
            // No working_directory override: the build tool's own registered config
            // (repo_path) resolves it — the branch already checked out by
            // CreateGitBranch is whatever is currently on disk there.
            let call_result = self.tools_client.execute_tool(build_tool_id, action, params, None).await;

            let (succeeded, output_json, error_text) = match &call_result {
                Ok(result) if result.success => (true, serde_json::json!({ "success": true, "output": result.output }), None),
                Ok(result) => {
                    let fix_prompt = format!(
                        "The {} step failed with this error. Propose a concrete fix.\n\nError:\n{}",
                        context_key,
                        result.error.clone().unwrap_or_else(|| result.output.clone()),
                    );
                    let fix_suggestion = self.agents_client.execute_agent(developer_agent_id, tenant_id, fix_prompt).await.ok();
                    (
                        false,
                        serde_json::json!({ "success": false, "output": result.output, "error": result.error, "fix_suggestion": fix_suggestion }),
                        result.error.clone(),
                    )
                }
                Err(e) => (false, serde_json::json!({ "success": false }), Some(e.to_string())),
            };

            step_execution.status = if succeeded { StepExecutionStatus::Succeeded } else { StepExecutionStatus::Failed };
            step_execution.output = Some(output_json.clone());
            step_execution.error = error_text;
            step_execution.completed_at = Some(Utc::now());
            step_execution.updated_at = Utc::now();

            run.merge_context(context_key, &output_json);
            run.updated_at = Utc::now();
            self.checkpoint_repository.save_checkpoint(&step_execution, run).await?;

            if succeeded {
                return Ok(());
            }
        }
        Err(DomainError::InternalError(format!("{} failed after {} attempts", context_key, MAX_FIX_ATTEMPTS)))
    }

    /// Drives the fixed pipeline from the roadmap diagram: Jira Ticket → Load
    /// Project Context → Retrieve Organizational Knowledge → Planner Agent →
    /// Architecture Review → Create Git Branch → Developer Agent → Compile (fix
    /// loop) → Run Tests (fix loop) → Reviewer Agent → Generate Documentation →
    /// Commit Changes → Push Branch → Create Pull Request → Update Jira.
    async fn execute_pipeline(&self, run: &mut SdlcRun, command: &StartSdlcRunCommand) -> Result<(), DomainError> {
        // 1. Fetch ticket (optional — only if a Jira tool is configured)
        let ticket_summary = if let Some(jira_tool_id) = command.jira_tool_id {
            let credentials = self.resolve_jira_credentials(jira_tool_id).await;
            let work = match credentials {
                Ok((email, api_token)) => {
                    let mut params = HashMap::new();
                    params.insert("issue_key".to_string(), command.ticket_key.clone());
                    params.insert("email".to_string(), email);
                    params.insert("api_token".to_string(), api_token);
                    self.tools_client.execute_tool(jira_tool_id, "get_issue".into(), params, None).await
                        .and_then(|r| if r.success {
                            Ok(serde_json::json!({ "ticket": r.output }))
                        } else {
                            Err(DomainError::InternalError(r.error.unwrap_or_else(|| "Jira fetch failed".into())))
                        })
                }
                Err(e) => Err(e),
            };
            let output = self.run_step(run, SdlcStep::FetchTicket, work).await?;
            output.get("ticket").and_then(|v| v.as_str()).map(str::to_string)
                .unwrap_or_else(|| command.ticket_summary.clone().unwrap_or_else(|| command.ticket_key.clone()))
        } else {
            command.ticket_summary.clone().unwrap_or_else(|| command.ticket_key.clone())
        };

        // 2. Load project context
        let project_work = self.platform_client.get_project(command.project_id).await
            .and_then(|ctx| serde_json::to_value(&ctx).map_err(|e| DomainError::InternalError(e.to_string())));
        let project_json = self.run_step(run, SdlcStep::LoadProjectContext, project_work).await?;
        let project_context: ProjectContext = serde_json::from_value(project_json)
            .map_err(|e| DomainError::InternalError(format!("Failed to parse project context: {}", e)))?;

        // 3. Retrieve organizational knowledge
        let knowledge_work = self.knowledge_client.search(command.tenant_id, ticket_summary.clone(), 5).await
            .and_then(|snippets| serde_json::to_value(&snippets).map_err(|e| DomainError::InternalError(e.to_string())));
        let knowledge_json = self.run_step(run, SdlcStep::RetrieveKnowledge, knowledge_work).await?;
        let snippets: Vec<KnowledgeSnippet> = serde_json::from_value(knowledge_json).unwrap_or_default();
        let knowledge_text = Self::knowledge_to_text(&snippets);

        // 4. Planner agent
        let planner_prompt = format!(
            "Ticket {}: {}\n\nRelevant organizational knowledge:\n{}\n\nProduce an implementation plan: tasks, risks, files to modify, acceptance criteria, and a complexity estimate.",
            command.ticket_key, ticket_summary, knowledge_text,
        );
        let planner_work = self.agents_client.execute_agent(command.planner_agent_id, command.tenant_id, planner_prompt).await
            .map(|text| serde_json::json!({ "plan": text }));
        let plan_json = self.run_step(run, SdlcStep::PlannerAgent, planner_work).await?;
        let plan_text = plan_json.get("plan").and_then(|v| v.as_str()).unwrap_or_default().to_string();

        // 5. Architecture review
        let architecture_prompt = format!("Review this implementation plan for architectural soundness and coding standards compliance:\n\n{}", plan_text);
        let architecture_work = self.agents_client.execute_agent(command.architect_agent_id, command.tenant_id, architecture_prompt).await
            .map(|text| serde_json::json!({ "review": text }));
        self.run_step(run, SdlcStep::ArchitectureReview, architecture_work).await?;

        // 6. Create git branch
        let branch_name = format!("sdlc/{}", command.ticket_key.to_lowercase());
        let mut branch_params = HashMap::new();
        branch_params.insert("branch".to_string(), branch_name.clone());
        branch_params.insert("create".to_string(), "true".to_string());
        let branch_work = self.tools_client.execute_tool(command.git_tool_id, "checkout".into(), branch_params, None).await
            .and_then(|r| if r.success {
                Ok(serde_json::json!({ "branch": branch_name }))
            } else {
                Err(DomainError::InternalError(r.error.unwrap_or_else(|| "Failed to create branch".into())))
            });
        self.run_step(run, SdlcStep::CreateGitBranch, branch_work).await?;
        run.branch_name = Some(branch_name.clone());
        self.run_repository.update(run).await?;

        // 7. Developer agent — asks the LLM for concrete file edits, then writes
        // them to the checked-out working directory via the Filesystem tool, so
        // Compile/Test/PR below operate on a real code change, not just the plan text.
        let developer_prompt = format!(
            "Implement the following plan on branch '{}'. Respond with STRICT JSON ONLY — no prose, no markdown code fences — \
             in exactly this shape: {{\"summary\": \"<what you changed and why>\", \"files\": [{{\"path\": \"<repo-relative path>\", \"content\": \"<complete new file contents>\"}}]}}. \
             Each file's `content` must be the file's ENTIRE new contents (not a diff/patch); include only files that need to change or be created.\n\nPlan:\n{}",
            branch_name, plan_text,
        );
        let developer_work = match self.agents_client.execute_agent(command.developer_agent_id, command.tenant_id, developer_prompt).await {
            Ok(text) => match Self::parse_file_edits(&text) {
                Some(edits) => {
                    let summary = Self::extract_json_object(&text)
                        .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
                        .and_then(|v| v.get("summary").and_then(|s| s.as_str()).map(str::to_string))
                        .unwrap_or_default();
                    self.apply_file_edits(command.filesystem_tool_id, &edits).await
                        .map(|paths| serde_json::json!({ "implementation_notes": summary, "files_written": paths }))
                }
                None => Err(DomainError::InternalError("Developer agent did not return valid file-edit JSON (expected {\"files\": [{\"path\":...,\"content\":...}]})".into())),
            },
            Err(e) => Err(e),
        };
        self.run_step(run, SdlcStep::DeveloperAgent, developer_work).await?;

        // 8/9. Compile, with a bounded "fix and retry" loop
        self.run_build_tool_with_fix_loop(
            run, SdlcStep::Compile, "build", "compile",
            command.build_tool_id, command.developer_agent_id, command.tenant_id,
            project_context.build_command.as_deref(),
        ).await?;

        // 10/11. Run tests, with a bounded "fix and retry" loop
        self.run_build_tool_with_fix_loop(
            run, SdlcStep::RunTests, "test", "run_tests",
            command.build_tool_id, command.developer_agent_id, command.tenant_id,
            project_context.test_command.as_deref(),
        ).await?;

        // 12. Reviewer agent
        let reviewer_prompt = format!(
            "Review the implementation on branch '{}' for coding standards, security issues, and architecture violations.\n\nOriginal plan:\n{}",
            branch_name, plan_text,
        );
        let reviewer_work = self.agents_client.execute_agent(command.reviewer_agent_id, command.tenant_id, reviewer_prompt).await
            .map(|text| serde_json::json!({ "review": text }));
        self.run_step(run, SdlcStep::ReviewerAgent, reviewer_work).await?;

        // 13. Generate documentation
        let docs_prompt = format!("Generate updated documentation (README/ADR/release notes) for ticket {}:\n\n{}", command.ticket_key, plan_text);
        let docs_work = self.agents_client.execute_agent(command.documentation_agent_id, command.tenant_id, docs_prompt).await
            .map(|text| serde_json::json!({ "documentation": text }));
        self.run_step(run, SdlcStep::GenerateDocumentation, docs_work).await?;

        // 14. Commit changes
        let mut commit_params = HashMap::new();
        commit_params.insert("message".to_string(), format!("{}: automated implementation via Autonomous SDLC", command.ticket_key));
        let commit_work = self.tools_client.execute_tool(command.git_tool_id, "commit".into(), commit_params, None).await
            .and_then(|r| if r.success { Ok(serde_json::json!({ "output": r.output })) } else { Err(DomainError::InternalError(r.error.unwrap_or_else(|| "Commit failed".into()))) });
        self.run_step(run, SdlcStep::CommitChanges, commit_work).await?;

        // 15. Push branch
        let push_work = self.tools_client.execute_tool(command.git_tool_id, "push".into(), HashMap::new(), None).await
            .and_then(|r| if r.success { Ok(serde_json::json!({ "output": r.output })) } else { Err(DomainError::InternalError(r.error.unwrap_or_else(|| "Push failed".into()))) });
        self.run_step(run, SdlcStep::PushBranch, push_work).await?;

        // 16. Create pull request (optional — only if a GitHub tool is configured)
        if let Some(github_tool_id) = command.github_tool_id {
            let token = self.resolve_github_token(github_tool_id).await;
            let pr_work = match token {
                Ok(token) => {
                    let mut pr_params = HashMap::new();
                    pr_params.insert("title".to_string(), format!("{}: automated implementation", command.ticket_key));
                    pr_params.insert("head".to_string(), branch_name.clone());
                    pr_params.insert("base".to_string(), project_context.default_branch.clone());
                    pr_params.insert("body".to_string(), plan_text.clone());
                    pr_params.insert("token".to_string(), token);
                    self.tools_client.execute_tool(github_tool_id, "create_pull_request".into(), pr_params, None).await
                        .and_then(|r| if r.success { Ok(serde_json::json!({ "output": r.output })) } else { Err(DomainError::InternalError(r.error.unwrap_or_else(|| "PR creation failed".into()))) })
                }
                Err(e) => Err(e),
            };
            let pr_json = self.run_step(run, SdlcStep::CreatePullRequest, pr_work).await?;
            if let Some(url) = Self::extract_pr_url(&pr_json) {
                run.pull_request_url = Some(url);
                self.run_repository.update(run).await?;
            }
        }

        // 17. Update Jira (optional — only if a Jira tool is configured)
        if let Some(jira_tool_id) = command.jira_tool_id {
            let credentials = self.resolve_jira_credentials(jira_tool_id).await;
            let jira_work = match credentials {
                Ok((email, api_token)) => {
                    let mut jira_params = HashMap::new();
                    jira_params.insert("issue_key".to_string(), command.ticket_key.clone());
                    jira_params.insert("email".to_string(), email);
                    jira_params.insert("api_token".to_string(), api_token);
                    let pr_note = run.pull_request_url.as_ref().map(|u| format!(" PR: {}", u)).unwrap_or_default();
                    jira_params.insert("comment".to_string(), format!("Implementation complete on branch '{}'.{}", branch_name, pr_note));
                    self.tools_client.execute_tool(jira_tool_id, "add_comment".into(), jira_params, None).await
                        .and_then(|r| if r.success { Ok(serde_json::json!({ "output": r.output })) } else { Err(DomainError::InternalError(r.error.unwrap_or_else(|| "Jira update failed".into()))) })
                }
                Err(e) => Err(e),
            };
            self.run_step(run, SdlcStep::UpdateJira, jira_work).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl SdlcRunPort for SdlcOrchestratorService {
    async fn start_run(&self, command: StartSdlcRunCommand) -> Result<SdlcRunResponse, DomainError> {
        if command.ticket_key.trim().is_empty() {
            return Err(DomainError::ValidationError("Ticket key cannot be empty".into()));
        }
        let mut run = SdlcRun::new(NewSdlcRun {
            tenant_id: command.tenant_id,
            project_id: command.project_id,
            ticket_key: command.ticket_key.clone(),
        });
        self.run_repository.save(&run).await?;
        info!(run_id = %run.id, ticket_key = %command.ticket_key, "Autonomous SDLC run started");

        match self.execute_pipeline(&mut run, &command).await {
            Ok(()) => {
                run.status = SdlcRunStatus::Completed;
                run.current_step = None;
                info!(run_id = %run.id, "Autonomous SDLC run completed");
            }
            Err(e) => {
                error!(run_id = %run.id, error = %e, "Autonomous SDLC run failed");
                run.status = SdlcRunStatus::Failed;
                run.error = Some(e.to_string());
            }
        }
        run.completed_at = Some(Utc::now());
        run.updated_at = Utc::now();
        self.run_repository.update(&run).await?;

        Ok(Self::to_response(&run))
    }

    async fn find_run_by_id(&self, id: Uuid) -> Result<SdlcRunResponse, DomainError> {
        let run = self.run_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("SDLC run not found: {}", id)))?;
        Ok(Self::to_response(&run))
    }

    async fn find_runs_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<SdlcRunResponse>, DomainError> {
        let runs = self.run_repository.find_by_tenant(tenant_id).await?;
        Ok(runs.iter().map(Self::to_response).collect())
    }

    async fn find_step_executions(&self, run_id: Uuid) -> Result<Vec<SdlcStepExecutionResponse>, DomainError> {
        let steps = self.step_repository.find_by_run_id(run_id).await?;
        Ok(steps.iter().map(Self::to_step_response).collect())
    }
}
