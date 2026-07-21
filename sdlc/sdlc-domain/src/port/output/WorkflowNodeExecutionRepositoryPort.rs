use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::WorkflowNodeExecution::WorkflowNodeExecution;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait WorkflowNodeExecutionRepositoryPort: Send + Sync {
    async fn save(&self, node_execution: &WorkflowNodeExecution) -> Result<(), DomainError>;
    async fn update(&self, node_execution: &WorkflowNodeExecution) -> Result<(), DomainError>;
    /// Every attempt of every node run so far for this execution — the checkpoint log
    /// the engine replays to compute what is already done and what is next.
    async fn find_by_execution_id(&self, execution_id: Uuid) -> Result<Vec<WorkflowNodeExecution>, DomainError>;
    /// The most recent attempt of one node within an execution (used to resolve
    /// pending approvals and to compute the next retry attempt number).
    async fn find_latest_by_execution_and_node(&self, execution_id: Uuid, node_id: &str) -> Result<Option<WorkflowNodeExecution>, DomainError>;
}
