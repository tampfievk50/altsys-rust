use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::RuleFiring::RuleFiring;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait RuleFiringRepositoryPort: Send + Sync {
    async fn save(&self, firing: &RuleFiring) -> Result<(), DomainError>;
    async fn find_by_event_id(&self, event_id: Uuid) -> Result<Vec<RuleFiring>, DomainError>;
}
