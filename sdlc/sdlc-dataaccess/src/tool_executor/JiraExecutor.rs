use std::time::Instant;

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use sdlc_domain::dto::ToolExecutionContext::ToolExecutionContext;
use sdlc_domain::dto::ToolExecutionResult::ToolExecutionResult;
use sdlc_domain::port::output::ToolExecutorPort::ToolExecutorPort;
use sdlc_domain::r#enum::DomainError::DomainError;

#[derive(Debug, Deserialize)]
struct JiraConfig {
    base_url: String,
    #[serde(default)]
    project_key: Option<String>,
}

pub struct JiraExecutor;

impl JiraExecutor {
    fn parse_config(context: &ToolExecutionContext) -> Result<JiraConfig, DomainError> {
        let raw = context.config.as_deref()
            .ok_or_else(|| DomainError::ValidationError("Jira tool requires config with 'base_url'".into()))?;
        serde_json::from_str(raw).map_err(|e| DomainError::ValidationError(format!("Invalid Jira tool config: {}", e)))
    }

    fn credentials(context: &ToolExecutionContext) -> Result<(&str, &str), DomainError> {
        let email = context.parameters.get("email")
            .ok_or_else(|| DomainError::ValidationError("Missing parameter 'email'".into()))?;
        let api_token = context.parameters.get("api_token")
            .ok_or_else(|| DomainError::ValidationError("Missing parameter 'api_token'".into()))?;
        Ok((email.as_str(), api_token.as_str()))
    }
}

#[async_trait]
impl ToolExecutorPort for JiraExecutor {
    fn tool_type(&self) -> &'static str {
        "jira"
    }

    async fn execute(&self, context: ToolExecutionContext) -> Result<ToolExecutionResult, DomainError> {
        let config = Self::parse_config(&context)?;
        let (email, api_token) = Self::credentials(&context)?;
        let client = reqwest::Client::new();
        let start = Instant::now();

        let response = match context.action.as_str() {
            "get_issue" => {
                let key = context.parameters.get("issue_key")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'issue_key'".into()))?;
                client
                    .get(format!("{}/rest/api/2/issue/{}", config.base_url, key))
                    .basic_auth(email, Some(api_token))
                    .send()
                    .await
            }
            "create_issue" => {
                let summary = context.parameters.get("summary")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'summary'".into()))?;
                let project_key = context.parameters.get("project_key").cloned()
                    .or_else(|| config.project_key.clone())
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'project_key'".into()))?;
                let issue_type = context.parameters.get("issue_type").cloned().unwrap_or_else(|| "Task".into());
                client
                    .post(format!("{}/rest/api/2/issue", config.base_url))
                    .basic_auth(email, Some(api_token))
                    .json(&json!({
                        "fields": {
                            "project": { "key": project_key },
                            "summary": summary,
                            "issuetype": { "name": issue_type }
                        }
                    }))
                    .send()
                    .await
            }
            "add_comment" => {
                let key = context.parameters.get("issue_key")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'issue_key'".into()))?;
                let comment = context.parameters.get("comment")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'comment'".into()))?;
                client
                    .post(format!("{}/rest/api/2/issue/{}/comment", config.base_url, key))
                    .basic_auth(email, Some(api_token))
                    .json(&json!({ "body": comment }))
                    .send()
                    .await
            }
            "search_issues" => {
                // Atlassian retired `GET /rest/api/2/search` (returns 410 Gone on
                // current Jira Cloud sites) in favor of this `/api/3` endpoint.
                let jql = context.parameters.get("jql")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'jql'".into()))?;
                client
                    .post(format!("{}/rest/api/3/search/jql", config.base_url))
                    .basic_auth(email, Some(api_token))
                    .json(&json!({
                        "jql": jql,
                        "fields": ["summary", "description", "issuetype", "priority", "updated"],
                        "maxResults": 50,
                    }))
                    .send()
                    .await
            }
            "update_issue" => {
                let key = context.parameters.get("issue_key")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'issue_key'".into()))?;
                let summary = context.parameters.get("summary")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'summary'".into()))?;
                client
                    .put(format!("{}/rest/api/3/issue/{}", config.base_url, key))
                    .basic_auth(email, Some(api_token))
                    .json(&json!({ "fields": { "summary": summary } }))
                    .send()
                    .await
            }
            "transition_issue" => {
                let key = context.parameters.get("issue_key")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'issue_key'".into()))?;
                let transition_id = context.parameters.get("transition_id")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'transition_id'".into()))?;
                client
                    .post(format!("{}/rest/api/2/issue/{}/transitions", config.base_url, key))
                    .basic_auth(email, Some(api_token))
                    .json(&json!({ "transition": { "id": transition_id } }))
                    .send()
                    .await
            }
            other => return Err(DomainError::ValidationError(format!("Unsupported jira action '{}'", other))),
        };

        let duration_ms = start.elapsed().as_millis() as i64;
        let response = response.map_err(|e| DomainError::InternalError(format!("Jira request failed: {}", e)))?;
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        if status.is_success() {
            Ok(ToolExecutionResult::ok(body, duration_ms))
        } else {
            Ok(ToolExecutionResult::failed(body, format!("Jira API returned {}", status), Some(status.as_u16() as i32), duration_ms))
        }
    }
}
