use async_trait::async_trait;

use sdlc_domain::dto::ToolExecutionContext::ToolExecutionContext;
use sdlc_domain::dto::ToolExecutionResult::ToolExecutionResult;
use sdlc_domain::port::output::ToolExecutorPort::ToolExecutorPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::tool_executor::CommandRunner::{resolve_working_directory, run_command, split_args};

/// Shared executor for CLI build tools (cargo, mvn, gradle). The concrete tools
/// (`CargoExecutor`, `MavenExecutor`, `GradleExecutor`) only differ in binary name
/// and the well-known action -> subcommand mapping.
pub struct BuildToolExecutor {
    tool_type: &'static str,
    binary: &'static str,
}

impl BuildToolExecutor {
    pub fn new(tool_type: &'static str, binary: &'static str) -> Self {
        Self { tool_type, binary }
    }

    fn base_args(&self, action: &str) -> Result<Vec<String>, DomainError> {
        let arg = match (self.binary, action) {
            (_, "run") => return Ok(Vec::new()), // caller supplies the full command via the 'args' parameter
            ("cargo", "build") => "build",
            ("cargo", "test") => "test",
            ("cargo", "clean") => "clean",
            ("mvn", "build") => "compile",
            ("mvn", "test") => "test",
            ("mvn", "clean") => "clean",
            ("mvn", "package") => "package",
            ("gradle", "build") => "build",
            ("gradle", "test") => "test",
            ("gradle", "clean") => "clean",
            (binary, other) => return Err(DomainError::ValidationError(format!("Unsupported {} action '{}'", binary, other))),
        };
        Ok(vec![arg.to_string()])
    }
}

#[async_trait]
impl ToolExecutorPort for BuildToolExecutor {
    fn tool_type(&self) -> &'static str {
        self.tool_type
    }

    async fn execute(&self, context: ToolExecutionContext) -> Result<ToolExecutionResult, DomainError> {
        let working_directory = resolve_working_directory(&context);
        let mut args = self.base_args(&context.action)?;
        if let Some(extra) = context.parameters.get("args") {
            args.extend(split_args(extra));
        }
        run_command(self.binary, &args, working_directory.as_deref()).await
    }
}
