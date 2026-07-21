use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::AutomationRule::AutomationRule;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait AutomationRuleRepositoryPort: Send + Sync {
    async fn save(&self, rule: &AutomationRule) -> Result<(), DomainError>;
    async fn update(&self, rule: &AutomationRule) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AutomationRule>, DomainError>;
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<AutomationRule>, DomainError>;
    /// Active rules for the tenant matching `event_type` — what `EventPort::ingest_event` evaluates.
    async fn find_active_by_tenant_and_event_type(&self, tenant_id: Uuid, event_type: &str) -> Result<Vec<AutomationRule>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;
}
