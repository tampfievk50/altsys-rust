use std::time::Instant;

use tokio::process::Command;

use sdlc_domain::dto::ToolExecutionContext::ToolExecutionContext;
use sdlc_domain::dto::ToolExecutionResult::ToolExecutionResult;
use sdlc_domain::r#enum::DomainError::DomainError;

/// Runs an external process to completion and converts its outcome into a
/// `ToolExecutionResult`. A non-zero exit code is a normal (failed) result, not
/// a `DomainError` — only the inability to spawn the process is an infra error.
pub async fn run_command(program: &str, args: &[String], working_directory: Option<&str>) -> Result<ToolExecutionResult, DomainError> {
    let start = Instant::now();
    let mut command = Command::new(program);
    command.args(args);
    if let Some(dir) = working_directory {
        command.current_dir(dir);
    }

    let output = command.output().await
        .map_err(|e| DomainError::InternalError(format!("Failed to spawn '{}': {}", program, e)))?;
    let duration_ms = start.elapsed().as_millis() as i64;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(ToolExecutionResult::ok(stdout, duration_ms))
    } else {
        Ok(ToolExecutionResult::failed(stdout, stderr, output.status.code(), duration_ms))
    }
}

/// Resolves the working directory for a process-based executor: the caller-supplied
/// `working_directory` wins; otherwise falls back to `working_directory` or
/// `repo_path` in the tool's own JSON config.
pub fn resolve_working_directory(context: &ToolExecutionContext) -> Option<String> {
    if let Some(dir) = &context.working_directory {
        return Some(dir.clone());
    }
    let config = context.config.as_ref()?;
    let value: serde_json::Value = serde_json::from_str(config).ok()?;
    value.get("working_directory")
        .or_else(|| value.get("repo_path"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

pub fn split_args(raw: &str) -> Vec<String> {
    raw.split_whitespace().map(|s| s.to_string()).collect()
}
