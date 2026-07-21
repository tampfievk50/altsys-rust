use std::path::{Path, PathBuf};
use std::time::Instant;

use async_trait::async_trait;
use tokio::fs;

use sdlc_domain::dto::ToolExecutionContext::ToolExecutionContext;
use sdlc_domain::dto::ToolExecutionResult::ToolExecutionResult;
use sdlc_domain::port::output::ToolExecutorPort::ToolExecutorPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::tool_executor::CommandRunner::resolve_working_directory;

pub struct FilesystemExecutor;

impl FilesystemExecutor {
    /// Resolves `relative_path` against `root`, rejecting anything that would
    /// escape the sandboxed root directory.
    fn resolve_path(root: &str, relative_path: &str) -> Result<PathBuf, DomainError> {
        if Path::new(relative_path).is_absolute() || relative_path.split('/').any(|seg| seg == "..") {
            return Err(DomainError::ValidationError("Path must be relative and cannot contain '..'".into()));
        }
        Ok(Path::new(root).join(relative_path))
    }
}

#[async_trait]
impl ToolExecutorPort for FilesystemExecutor {
    fn tool_type(&self) -> &'static str {
        "filesystem"
    }

    async fn execute(&self, context: ToolExecutionContext) -> Result<ToolExecutionResult, DomainError> {
        let root = resolve_working_directory(&context)
            .ok_or_else(|| DomainError::ValidationError("Filesystem tool requires a working directory (config.working_directory or request working_directory)".into()))?;
        let start = Instant::now();

        match context.action.as_str() {
            "list" => {
                let relative = context.parameters.get("path").cloned().unwrap_or_default();
                let dir = Self::resolve_path(&root, &relative)?;
                let mut entries = fs::read_dir(&dir).await
                    .map_err(|e| DomainError::InternalError(format!("Failed to list '{}': {}", dir.display(), e)))?;
                let mut names = Vec::new();
                while let Some(entry) = entries.next_entry().await.map_err(|e| DomainError::InternalError(e.to_string()))? {
                    names.push(entry.file_name().to_string_lossy().to_string());
                }
                Ok(ToolExecutionResult::ok(names.join("\n"), start.elapsed().as_millis() as i64))
            }
            "read" => {
                let relative = context.parameters.get("path")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'path'".into()))?;
                let file = Self::resolve_path(&root, relative)?;
                match fs::read_to_string(&file).await {
                    Ok(content) => Ok(ToolExecutionResult::ok(content, start.elapsed().as_millis() as i64)),
                    Err(e) => Ok(ToolExecutionResult::failed(String::new(), e.to_string(), None, start.elapsed().as_millis() as i64)),
                }
            }
            "write" => {
                let relative = context.parameters.get("path")
                    .ok_or_else(|| DomainError::ValidationError("Missing parameter 'path'".into()))?;
                let content = context.parameters.get("content").cloned().unwrap_or_default();
                let file = Self::resolve_path(&root, relative)?;
                if let Some(parent) = file.parent() {
                    fs::create_dir_all(parent).await
                        .map_err(|e| DomainError::InternalError(format!("Failed to create '{}': {}", parent.display(), e)))?;
                }
                match fs::write(&file, &content).await {
                    Ok(_) => Ok(ToolExecutionResult::ok(format!("Wrote {} bytes to {}", content.len(), file.display()), start.elapsed().as_millis() as i64)),
                    Err(e) => Ok(ToolExecutionResult::failed(String::new(), e.to_string(), None, start.elapsed().as_millis() as i64)),
                }
            }
            other => Err(DomainError::ValidationError(format!("Unsupported filesystem action '{}'", other))),
        }
    }
}
