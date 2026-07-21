use std::time::Instant;

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use sdlc_domain::dto::ToolExecutionContext::ToolExecutionContext;
use sdlc_domain::dto::ToolExecutionResult::ToolExecutionResult;
use sdlc_domain::port::output::ToolExecutorPort::ToolExecutorPort;
use sdlc_domain::r#enum::DomainError::DomainError;

#[derive(Debug, Deserialize)]
struct GitHubConfig {
    /// `"owner/name"`, matching how Settings → GitHub in the admin panel stores
    /// it — split into the two path segments the GitHub API needs at call time.
    repository: String,
}

impl GitHubConfig {
    fn owner_repo(&self) -> Result<(&str, &str), DomainError> {
        self.repository.split_once('/')
            .ok_or_else(|| DomainError::ValidationError(format!("GitHub tool config 'repository' must be 'owner/repo', got '{}'", self.repository)))
    }
}

pub struct GitHubExecutor;

impl GitHubExecutor {
    fn parse_config(context: &ToolExecutionContext) -> Result<GitHubConfig, DomainError> {
        let raw = context.config.as_deref()
            .ok_or_else(|| DomainError::ValidationError("GitHub tool requires config with 'repository' (owner/name)".into()))?;
        serde_json::from_str(raw).map_err(|e| DomainError::ValidationError(format!("Invalid GitHub tool config: {}", e)))
    }

    fn token(context: &ToolExecutionContext) -> Result<&str, DomainError> {
        context.parameters.get("token").map(String::as_str)
            .ok_or_else(|| DomainError::ValidationError("Missing parameter 'token' (GitHub personal access token)".into()))
    }
}

#[async_trait]
impl ToolExecutorPort for GitHubExecutor {
    fn tool_type(&self) -> &'static str {
        "github"
    }

    async fn execute(&self, context: ToolExecutionContext) -> Result<ToolExecutionResult, DomainError> {
        let config = Self::parse_config(&context)?;
        let (owner, repo) = config.owner_repo()?;
        let token = Self::token(&context)?;
        let client = reqwest::Client::new();
        let start = Instant::now();

        let response = match context.action.as_str() {
            "get_repo" => client
                .get(format!("https://api.github.com/repos/{}/{}", owner, repo))
                .header("Authorization", format!("Bearer {}", token))
                .header("User-Agent", "altsys-tools-service")
                .send()
                .await,
            "list_issues" => client
                .get(format!("https://api.github.com/repos/{}/{}/issues", owner, repo))
                .header("Authorization", format!("Bearer {}", token))
                .header("User-Agent", "altsys-tools-service")
                .send()
                .await,
            "create_issue" => {
                let title = context.parameters.get("title")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'title'".into()))?;
                let body = context.parameters.get("body").cloned().unwrap_or_default();
                client
                    .post(format!("https://api.github.com/repos/{}/{}/issues", owner, repo))
                    .header("Authorization", format!("Bearer {}", token))
                    .header("User-Agent", "altsys-tools-service")
                    .json(&json!({ "title": title, "body": body }))
                    .send()
                    .await
            }
            "create_pull_request" => {
                let title = context.parameters.get("title")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'title'".into()))?;
                let head = context.parameters.get("head")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'head'".into()))?;
                let base = context.parameters.get("base")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'base'".into()))?;
                let body = context.parameters.get("body").cloned().unwrap_or_default();
                client
                    .post(format!("https://api.github.com/repos/{}/{}/pulls", owner, repo))
                    .header("Authorization", format!("Bearer {}", token))
                    .header("User-Agent", "altsys-tools-service")
                    .json(&json!({ "title": title, "head": head, "base": base, "body": body }))
                    .send()
                    .await
            }
            other => return Err(DomainError::ValidationError(format!("Unsupported github action '{}'", other))),
        };

        let duration_ms = start.elapsed().as_millis() as i64;
        let response = response.map_err(|e| DomainError::InternalError(format!("GitHub request failed: {}", e)))?;
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        if status.is_success() {
            Ok(ToolExecutionResult::ok(body, duration_ms))
        } else {
            Ok(ToolExecutionResult::failed(body, format!("GitHub API returned {}", status), Some(status.as_u16() as i32), duration_ms))
        }
    }
}
