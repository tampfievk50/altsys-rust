use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::CreateUserCommand::CreateUserCommand;
use crate::dto::UpdateUserCommand::UpdateUserCommand;
use crate::dto::UserResponse::UserResponse;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait UserPort: Send + Sync {
    async fn create_user(&self, command: CreateUserCommand) -> Result<UserResponse, DomainError>;
    async fn find_user_by_id(&self, id: Uuid) -> Result<UserResponse, DomainError>;
    async fn find_all_users(&self) -> Result<Vec<UserResponse>, DomainError>;
    async fn update_user(&self, id: Uuid, command: UpdateUserCommand) -> Result<UserResponse, DomainError>;
    async fn delete_user(&self, id: Uuid) -> Result<(), DomainError>;
}
