use async_trait::async_trait;

use sdlc_domain::dto::ToolExecutionContext::ToolExecutionContext;
use sdlc_domain::dto::ToolExecutionResult::ToolExecutionResult;
use sdlc_domain::port::output::ToolExecutorPort::ToolExecutorPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::tool_executor::CommandRunner::{resolve_working_directory, run_command};

pub struct GitExecutor;

#[async_trait]
impl ToolExecutorPort for GitExecutor {
    fn tool_type(&self) -> &'static str {
        "git"
    }

    async fn execute(&self, context: ToolExecutionContext) -> Result<ToolExecutionResult, DomainError> {
        let working_directory = resolve_working_directory(&context);

        let args: Vec<String> = match context.action.as_str() {
            "clone" => {
                let url = context.parameters.get("repository_url")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'repository_url'".into()))?;
                let mut args = vec!["clone".to_string(), url.clone()];
                if let Some(target) = context.parameters.get("target_dir") {
                    args.push(target.clone());
                }
                args
            }
            "status" => vec!["status".into(), "--short".into(), "--branch".into()],
            "pull" => vec!["pull".into()],
            "push" => vec!["push".into()],
            "checkout" => {
                let branch = context.parameters.get("branch")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'branch'".into()))?;
                let create = context.parameters.get("create").map(String::as_str) == Some("true");
                if create {
                    vec!["checkout".into(), "-b".into(), branch.clone()]
                } else {
                    vec!["checkout".into(), branch.clone()]
                }
            }
            "commit" => {
                let message = context.parameters.get("message")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'message'".into()))?;
                vec!["commit".into(), "-am".into(), message.clone()]
            }
            "log" => {
                let limit = context.parameters.get("limit").cloned().unwrap_or_else(|| "10".into());
                vec!["log".into(), "--oneline".into(), "-n".into(), limit]
            }
            other => return Err(DomainError::ValidationError(format!("Unsupported git action '{}'", other))),
        };

        run_command("git", &args, working_directory.as_deref()).await
    }
}
