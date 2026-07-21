use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::AgentExecution::AgentExecution;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait AgentExecutionRepositoryPort: Send + Sync {
    async fn save(&self, execution: &AgentExecution) -> Result<(), DomainError>;
    async fn update(&self, execution: &AgentExecution) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AgentExecution>, DomainError>;
    async fn find_by_agent_id(&self, agent_id: Uuid) -> Result<Vec<AgentExecution>, DomainError>;
}
