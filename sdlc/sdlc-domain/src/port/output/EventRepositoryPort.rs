use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::IngestedEvent::IngestedEvent;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait EventRepositoryPort: Send + Sync {
    async fn save(&self, event: &IngestedEvent) -> Result<(), DomainError>;
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<IngestedEvent>, DomainError>;
}
