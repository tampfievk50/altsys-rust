use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::Permission::Permission;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait PermissionRepositoryPort: Send + Sync {
    async fn save(&self, permission: &Permission) -> Result<(), DomainError>;
    async fn update(&self, permission: &Permission) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Permission>, DomainError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Permission>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Permission>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;
}
