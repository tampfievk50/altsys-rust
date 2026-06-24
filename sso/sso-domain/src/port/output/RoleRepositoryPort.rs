use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::Role::Role;
use crate::dto::RolePermission::RolePermission;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait RoleRepositoryPort: Send + Sync {
    async fn save(&self, role: &Role) -> Result<(), DomainError>;
    async fn update(&self, role: &Role) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>, DomainError>;
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Role>, DomainError>;
    async fn find_by_name_and_tenant(&self, name: &str, tenant_id: Uuid) -> Result<Option<Role>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;
    async fn save_role_permission(&self, rp: &RolePermission) -> Result<(), DomainError>;
    async fn delete_role_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<bool, DomainError>;
    async fn find_permissions_by_role(&self, role_id: Uuid) -> Result<Vec<RolePermission>, DomainError>;
}
