use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::CreateRoleCommand::CreateRoleCommand;
use crate::dto::UpdateRoleCommand::UpdateRoleCommand;
use crate::dto::RoleResponse::RoleResponse;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait RolePort: Send + Sync {
    async fn create_role(&self, command: CreateRoleCommand) -> Result<RoleResponse, DomainError>;
    async fn find_role_by_id(&self, id: Uuid) -> Result<RoleResponse, DomainError>;
    async fn find_roles_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<RoleResponse>, DomainError>;
    async fn update_role(&self, id: Uuid, command: UpdateRoleCommand) -> Result<RoleResponse, DomainError>;
    async fn delete_role(&self, id: Uuid) -> Result<(), DomainError>;
    async fn assign_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), DomainError>;
    async fn remove_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), DomainError>;
}
