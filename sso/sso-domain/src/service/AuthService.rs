use std::env;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use tracing::info;
use uuid::Uuid;

use crate::dto::Claims::Claims;
use crate::dto::LoginCommand::LoginCommand;
use crate::dto::RefreshToken::RefreshToken;
use crate::dto::TokenResponse::TokenResponse;
use crate::port::input::AuthPort::AuthPort;
use crate::port::output::RefreshTokenRepositoryPort::RefreshTokenRepositoryPort;
use crate::port::output::TenantRepositoryPort::TenantRepositoryPort;
use crate::port::output::UserRepositoryPort::UserRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct AuthService {
    user_repository: Arc<dyn UserRepositoryPort>,
    tenant_repository: Arc<dyn TenantRepositoryPort>,
    refresh_token_repository: Arc<dyn RefreshTokenRepositoryPort>,
}

impl AuthService {
    pub fn new(
        user_repository: Arc<dyn UserRepositoryPort>,
        tenant_repository: Arc<dyn TenantRepositoryPort>,
        refresh_token_repository: Arc<dyn RefreshTokenRepositoryPort>,
    ) -> Self {
        Self { user_repository, tenant_repository, refresh_token_repository }
    }

    fn jwt_secret() -> String {
        env::var("JWT_SECRET").unwrap_or_else(|_| "default-secret-change-me".into())
    }

    fn access_expiry_seconds() -> i64 {
        env::var("JWT_EXPIRY_SECONDS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3600)
    }

    fn refresh_expiry_seconds() -> i64 {
        env::var("JWT_REFRESH_EXPIRY_SECONDS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(604800)
    }

    fn create_access_token(user_id: Uuid, tenant_id: Uuid, username: &str) -> Result<String, DomainError> {
        let secret = Self::jwt_secret();
        let now = Utc::now();
        let exp = (now + Duration::seconds(Self::access_expiry_seconds())).timestamp() as usize;
        let claims = Claims {
            sub: user_id.to_string(),
            tenant_id: tenant_id.to_string(),
            username: username.to_string(),
            exp,
            iat: now.timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| DomainError::InternalError(format!("Failed to create token: {}", e)))
    }

    pub fn validate_token(token: &str) -> Result<Claims, DomainError> {
        let secret = Self::jwt_secret();
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| DomainError::Unauthorized(format!("Invalid token: {}", e)))
    }
}

#[async_trait]
impl AuthPort for AuthService {
    async fn login(&self, command: LoginCommand) -> Result<TokenResponse, DomainError> {
        info!(username = %command.username, tenant = %command.tenant_slug, "Login attempt");

        let tenant = self.tenant_repository.find_by_slug(&command.tenant_slug).await?
            .ok_or_else(|| DomainError::Unauthorized("Invalid credentials".into()))?;

        if !tenant.is_active {
            return Err(DomainError::Forbidden("Tenant is inactive".into()));
        }

        let user = self.user_repository
            .find_by_username_and_tenant(&command.username, tenant.id)
            .await?
            .ok_or_else(|| DomainError::Unauthorized("Invalid credentials".into()))?;

        if !user.is_active {
            return Err(DomainError::Forbidden("User account is inactive".into()));
        }

        let valid = bcrypt::verify(&command.password, &user.password_hash)
            .map_err(|e| DomainError::InternalError(format!("Password verification failed: {}", e)))?;

        if !valid {
            return Err(DomainError::Unauthorized("Invalid credentials".into()));
        }

        let access_token = Self::create_access_token(user.id, tenant.id, &user.username)?;

        let refresh_token_value = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::seconds(Self::refresh_expiry_seconds());
        let refresh_token = RefreshToken::new(user.id, tenant.id, refresh_token_value.clone(), expires_at);
        self.refresh_token_repository.save(&refresh_token).await?;

        info!(user_id = %user.id, "Login successful");

        Ok(TokenResponse {
            access_token,
            refresh_token: refresh_token_value,
            token_type: "Bearer".into(),
            expires_in: Self::access_expiry_seconds(),
        })
    }

    async fn refresh(&self, refresh_token: &str) -> Result<TokenResponse, DomainError> {
        let stored = self.refresh_token_repository.find_by_token(refresh_token).await?
            .ok_or_else(|| DomainError::Unauthorized("Invalid refresh token".into()))?;

        if stored.is_revoked {
            return Err(DomainError::Unauthorized("Refresh token has been revoked".into()));
        }
        if stored.expires_at < Utc::now() {
            return Err(DomainError::Unauthorized("Refresh token has expired".into()));
        }

        let user = self.user_repository.find_by_id(stored.user_id).await?
            .ok_or_else(|| DomainError::Unauthorized("User not found".into()))?;

        self.refresh_token_repository.revoke(refresh_token).await?;

        let access_token = Self::create_access_token(user.id, stored.tenant_id, &user.username)?;
        let new_refresh_value = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::seconds(Self::refresh_expiry_seconds());
        let new_refresh = RefreshToken::new(user.id, stored.tenant_id, new_refresh_value.clone(), expires_at);
        self.refresh_token_repository.save(&new_refresh).await?;

        Ok(TokenResponse {
            access_token,
            refresh_token: new_refresh_value,
            token_type: "Bearer".into(),
            expires_in: Self::access_expiry_seconds(),
        })
    }

    async fn logout(&self, refresh_token: &str) -> Result<(), DomainError> {
        let revoked = self.refresh_token_repository.revoke(refresh_token).await?;
        if !revoked {
            return Err(DomainError::NotFound("Refresh token not found".into()));
        }
        info!("Logout: refresh token revoked");
        Ok(())
    }
}
