use async_trait::async_trait;

use crate::dto::ToolExecutionContext::ToolExecutionContext;
use crate::dto::ToolExecutionResult::ToolExecutionResult;
use crate::r#enum::DomainError::DomainError;

/// A driven adapter that knows how to carry out actions for one built-in tool
/// type (git, github, jira, filesystem, cargo, maven, gradle, ...). Implementations
/// live in `tools-dataaccess/src/executor/` since they talk to external systems
/// (processes, HTTP APIs, the filesystem) rather than the database.
#[async_trait]
pub trait ToolExecutorPort: Send + Sync {
    /// Matches `Tool::tool_type` in the registry.
    fn tool_type(&self) -> &'static str;

    async fn execute(&self, context: ToolExecutionContext) -> Result<ToolExecutionResult, DomainError>;
}
