use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::Model::Model;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait ModelRepositoryPort: Send + Sync {
    async fn save(&self, model: &Model) -> Result<(), DomainError>;
    async fn update(&self, model: &Model) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, DomainError>;
    /// Returns rows where `tenant_id = tenant_id` OR `tenant_id IS NULL` (global models).
    async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Model>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;
}
