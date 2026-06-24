use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::Tenant::Tenant;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait TenantRepositoryPort: Send + Sync {
    async fn save(&self, tenant: &Tenant) -> Result<(), DomainError>;
    async fn update(&self, tenant: &Tenant) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tenant>, DomainError>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<Tenant>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Tenant>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;
}
