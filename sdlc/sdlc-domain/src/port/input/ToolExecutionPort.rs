use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::ExecuteToolCommand::ExecuteToolCommand;
use crate::dto::ToolExecutionResult::ToolExecutionResult;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait ToolExecutionPort: Send + Sync {
    /// Looks up the registered tool by id, dispatches to the executor matching its
    /// `tool_type`, and runs `command.action` against it.
    async fn execute_tool(&self, tool_id: Uuid, command: ExecuteToolCommand) -> Result<ToolExecutionResult, DomainError>;
}
