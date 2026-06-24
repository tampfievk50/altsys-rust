use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::CreatePermissionCommand::CreatePermissionCommand;
use crate::dto::UpdatePermissionCommand::UpdatePermissionCommand;
use crate::dto::PermissionResponse::PermissionResponse;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait PermissionPort: Send + Sync {
    async fn create_permission(&self, command: CreatePermissionCommand) -> Result<PermissionResponse, DomainError>;
    async fn find_permission_by_id(&self, id: Uuid) -> Result<PermissionResponse, DomainError>;
    async fn find_all_permissions(&self) -> Result<Vec<PermissionResponse>, DomainError>;
    async fn update_permission(&self, id: Uuid, command: UpdatePermissionCommand) -> Result<PermissionResponse, DomainError>;
    async fn delete_permission(&self, id: Uuid) -> Result<(), DomainError>;
}
