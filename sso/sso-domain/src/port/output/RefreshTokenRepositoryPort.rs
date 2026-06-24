use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::RefreshToken::RefreshToken;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait RefreshTokenRepositoryPort: Send + Sync {
    async fn save(&self, token: &RefreshToken) -> Result<(), DomainError>;
    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>, DomainError>;
    async fn revoke(&self, token: &str) -> Result<bool, DomainError>;
    async fn revoke_all_for_user(&self, user_id: Uuid) -> Result<(), DomainError>;
}
