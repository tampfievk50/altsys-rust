use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::dto::ExecuteToolCommand::ExecuteToolCommand;
use crate::dto::IngestEventCommand::IngestEventCommand;
use crate::dto::Project::Project;
use crate::port::input::CredentialPort::CredentialPort;
use crate::port::input::EventPort::EventPort;
use crate::port::input::ToolExecutionPort::ToolExecutionPort;
use crate::port::output::ProjectRepositoryPort::ProjectRepositoryPort;
use crate::port::output::ToolRepositoryPort::ToolRepositoryPort;
use crate::r#enum::DomainError::DomainError;

/// Fallback ingestion path for tenants whose Jira instance can't reach our
/// webhook endpoint (see `JiraWebhookController`, the preferred real-time path).
/// On each tick, polls every project with a Jira config that has no
/// `webhook_secret` set, searches for tickets updated since the project's last
/// successful poll, and feeds them through the same `EventPort::ingest_event`
/// the webhook uses — so automation rules see identical events either way.
pub struct JiraPollingScheduler {
    project_repository: Arc<dyn ProjectRepositoryPort>,
    tool_repository: Arc<dyn ToolRepositoryPort>,
    credential_port: Arc<dyn CredentialPort>,
    tool_execution_port: Arc<dyn ToolExecutionPort>,
    event_port: Arc<dyn EventPort>,
}

impl JiraPollingScheduler {
    pub fn new(
        project_repository: Arc<dyn ProjectRepositoryPort>,
        tool_repository: Arc<dyn ToolRepositoryPort>,
        credential_port: Arc<dyn CredentialPort>,
        tool_execution_port: Arc<dyn ToolExecutionPort>,
        event_port: Arc<dyn EventPort>,
    ) -> Self {
        Self { project_repository, tool_repository, credential_port, tool_execution_port, event_port }
    }

    /// Runs forever on the given cadence; call via `tokio::spawn`. Each tick's
    /// failure is logged and swallowed so one bad cycle doesn't kill the loop.
    pub async fn run_forever(self: Arc<Self>, interval: Duration) {
        let mut ticker = tokio::time::interval(interval);
        loop {
            ticker.tick().await;
            if let Err(e) = self.poll_once().await {
                error!(error = %e, "Jira polling cycle failed");
            }
        }
    }

    pub async fn poll_once(&self) -> Result<(), DomainError> {
        let projects = self.project_repository.find_all_with_jira_tool().await?;
        for project in projects {
            let Some(jira_tool_id) = project.jira_tool_id else { continue };
            if let Err(e) = self.poll_project(&project, jira_tool_id).await {
                warn!(project_id = %project.id, error = %e, "Failed to poll Jira for project");
            }
        }
        Ok(())
    }

    async fn poll_project(&self, project: &Project, jira_tool_id: Uuid) -> Result<(), DomainError> {
        let tool = self.tool_repository.find_by_id(jira_tool_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Jira tool not found: {}", jira_tool_id)))?;
        if !tool.is_enabled {
            return Ok(());
        }

        let config: serde_json::Value = tool.config.as_deref()
            .and_then(|c| serde_json::from_str(c).ok())
            .unwrap_or_default();

        // A configured webhook is the preferred, real-time path — skip polling
        // so the same ticket isn't ingested twice.
        if config.get("webhook_secret").and_then(|v| v.as_str()).is_some() {
            return Ok(());
        }

        let project_key = config.get("project_key").and_then(|v| v.as_str())
            .ok_or_else(|| DomainError::ValidationError("Jira tool config missing 'project_key'".into()))?;
        let email = config.get("email").and_then(|v| v.as_str())
            .ok_or_else(|| DomainError::ValidationError("Jira tool config missing 'email'".into()))?;
        let credential_id: Uuid = config.get("credential_id").and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| DomainError::ValidationError("Jira tool config missing 'credential_id'".into()))?;
        let api_token = self.credential_port.reveal_credential_secret(credential_id).await?.secret;

        // First sync (no cursor yet) pulls the whole current backlog rather than
        // just the last 24h — a project just connected to Jira should see its
        // existing tickets, not only what changes from here on.
        let jql = match project.jira_last_synced_at {
            Some(since) => format!(
                r#"project = {} AND updated > "{}" ORDER BY updated ASC"#,
                project_key,
                since.format("%Y-%m-%d %H:%M"),
            ),
            None => format!("project = {} ORDER BY updated ASC", project_key),
        };

        let mut parameters = HashMap::new();
        parameters.insert("jql".to_string(), jql);
        parameters.insert("email".to_string(), email.to_string());
        parameters.insert("api_token".to_string(), api_token);

        let result = self.tool_execution_port.execute_tool(jira_tool_id, ExecuteToolCommand {
            action: "search_issues".to_string(),
            parameters,
            working_directory: None,
        }).await?;

        if !result.success {
            return Err(DomainError::InternalError(result.error.unwrap_or_else(|| "Jira search failed".into())));
        }

        let body: serde_json::Value = serde_json::from_str(&result.output)
            .map_err(|e| DomainError::InternalError(format!("Invalid Jira search response: {}", e)))?;
        let issues = body.get("issues").and_then(|v| v.as_array()).cloned().unwrap_or_default();

        for issue in &issues {
            let event_payload = serde_json::json!({
                "project_id": project.id,
                "ticket_key": issue.get("key"),
                "summary": issue.pointer("/fields/summary"),
                "description": issue.pointer("/fields/description"),
                "issue_type": issue.pointer("/fields/issuetype/name"),
                "priority": issue.pointer("/fields/priority/name"),
            });
            self.event_port.ingest_event(IngestEventCommand {
                tenant_id: project.tenant_id,
                event_type: "jira.ticket.updated".to_string(),
                payload: event_payload,
            }).await?;
        }

        info!(project_id = %project.id, count = issues.len(), "Polled Jira for project");

        if let Some(mut updated) = self.project_repository.find_by_id(project.id).await? {
            updated.jira_last_synced_at = Some(Utc::now());
            self.project_repository.update(&updated).await?;
        }
        Ok(())
    }
}
