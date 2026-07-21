use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::AgentResponse::AgentResponse;
use crate::dto::CreateAgentCommand::CreateAgentCommand;
use crate::dto::UpdateAgentCommand::UpdateAgentCommand;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait AgentPort: Send + Sync {
    async fn create_agent(&self, command: CreateAgentCommand) -> Result<AgentResponse, DomainError>;
    async fn find_agent_by_id(&self, id: Uuid) -> Result<AgentResponse, DomainError>;
    /// Returns agents scoped to `tenant_id` plus platform-wide (tenant_id = NULL) agents.
    async fn find_agents_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<AgentResponse>, DomainError>;
    async fn update_agent(&self, id: Uuid, command: UpdateAgentCommand) -> Result<AgentResponse, DomainError>;
    async fn delete_agent(&self, id: Uuid) -> Result<(), DomainError>;
}
