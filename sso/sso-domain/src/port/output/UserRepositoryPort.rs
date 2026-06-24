use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::User::User;
use crate::dto::UserRole::UserRole;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait UserRepositoryPort: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn update(&self, user: &User) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn find_by_username_and_tenant(&self, username: &str, tenant_id: Uuid) -> Result<Option<User>, DomainError>;
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<User>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;
    async fn save_user_role(&self, user_role: &UserRole) -> Result<(), DomainError>;
    async fn delete_user_role(&self, user_id: Uuid, role_id: Uuid) -> Result<bool, DomainError>;
    async fn find_roles_by_user(&self, user_id: Uuid) -> Result<Vec<UserRole>, DomainError>;
}
