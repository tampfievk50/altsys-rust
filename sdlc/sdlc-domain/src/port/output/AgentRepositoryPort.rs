use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::Agent::Agent;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait AgentRepositoryPort: Send + Sync {
    async fn save(&self, agent: &Agent) -> Result<(), DomainError>;
    async fn update(&self, agent: &Agent) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Agent>, DomainError>;
    /// Returns rows where `tenant_id = tenant_id` OR `tenant_id IS NULL` (global agents).
    async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Agent>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;
}
