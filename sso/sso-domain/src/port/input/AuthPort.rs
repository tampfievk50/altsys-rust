use async_trait::async_trait;

use crate::dto::LoginCommand::LoginCommand;
use crate::dto::TokenResponse::TokenResponse;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait AuthPort: Send + Sync {
    async fn login(&self, command: LoginCommand) -> Result<TokenResponse, DomainError>;
    async fn refresh(&self, refresh_token: &str) -> Result<TokenResponse, DomainError>;
    async fn logout(&self, refresh_token: &str) -> Result<(), DomainError>;
}
