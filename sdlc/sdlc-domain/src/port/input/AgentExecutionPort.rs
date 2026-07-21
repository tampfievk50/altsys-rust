use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::AgentExecutionResponse::AgentExecutionResponse;
use crate::dto::ExecuteAgentCommand::ExecuteAgentCommand;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait AgentExecutionPort: Send + Sync {
    /// Looks up the agent, dispatches to the `LlmClientPort` matching its `provider`,
    /// and persists an execution record for the call.
    async fn execute_agent(&self, agent_id: Uuid, command: ExecuteAgentCommand) -> Result<AgentExecutionResponse, DomainError>;
    async fn find_execution_by_id(&self, id: Uuid) -> Result<AgentExecutionResponse, DomainError>;
    async fn find_executions_by_agent(&self, agent_id: Uuid) -> Result<Vec<AgentExecutionResponse>, DomainError>;
}
