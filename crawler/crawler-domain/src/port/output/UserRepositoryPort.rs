use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::User::User;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait UserRepositoryPort: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn update(&self, user: &User) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn find_all(&self) -> Result<Vec<User>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;
}
