use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::ApprovalDecisionCommand::ApprovalDecisionCommand;
use crate::dto::StartWorkflowExecutionCommand::StartWorkflowExecutionCommand;
use crate::dto::WorkflowExecutionResponse::WorkflowExecutionResponse;
use crate::dto::WorkflowNodeExecutionResponse::WorkflowNodeExecutionResponse;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait WorkflowExecutionPort: Send + Sync {
    /// Starts the Start node and runs the graph forward (via `continue_execution`)
    /// until it completes, fails, or reaches an approval gate.
    async fn start_execution(&self, command: StartWorkflowExecutionCommand) -> Result<WorkflowExecutionResponse, DomainError>;
    async fn find_execution_by_id(&self, id: Uuid) -> Result<WorkflowExecutionResponse, DomainError>;
    async fn find_executions_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WorkflowExecutionResponse>, DomainError>;
    /// Checkpoint history: every attempt of every node that has run for this execution.
    async fn find_node_executions(&self, execution_id: Uuid) -> Result<Vec<WorkflowNodeExecutionResponse>, DomainError>;
    /// Resolves a pending `approval` node and resumes the graph from checkpoint state.
    async fn decide_approval(&self, execution_id: Uuid, node_id: &str, command: ApprovalDecisionCommand) -> Result<WorkflowExecutionResponse, DomainError>;
}
