use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sso_domain::dto::RefreshToken::RefreshToken;
use sso_domain::port::output::RefreshTokenRepositoryPort::RefreshTokenRepositoryPort;
use sso_domain::r#enum::DomainError::DomainError;

use crate::refresh_token::entity::RefreshTokenEntity;
use crate::refresh_token::repository::RefreshTokenSeaOrmRepository::RefreshTokenSeaOrmRepository;
use sea_orm::Set;

pub struct RefreshTokenRepositoryImpl {
    sea_orm_repo: RefreshTokenSeaOrmRepository,
}

impl RefreshTokenRepositoryImpl {
    pub fn new(sea_orm_repo: RefreshTokenSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }

    fn to_active_model(token: &RefreshToken) -> RefreshTokenEntity::ActiveModel {
        RefreshTokenEntity::ActiveModel {
            id: Set(token.id),
            user_id: Set(token.user_id),
            tenant_id: Set(token.tenant_id),
            token: Set(token.token.clone()),
            expires_at: Set(token.expires_at),
            is_revoked: Set(token.is_revoked),
            created_at: Set(token.created_at),
        }
    }

    fn to_domain(model: &RefreshTokenEntity::Model) -> RefreshToken {
        RefreshToken {
            id: model.id,
            user_id: model.user_id,
            tenant_id: model.tenant_id,
            token: model.token.clone(),
            expires_at: model.expires_at,
            is_revoked: model.is_revoked,
            created_at: model.created_at,
        }
    }
}

#[async_trait]
impl RefreshTokenRepositoryPort for RefreshTokenRepositoryImpl {
    async fn save(&self, token: &RefreshToken) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(Self::to_active_model(token)).await
            .map(|_| ()).map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>, DomainError> {
        self.sea_orm_repo.find_by_token(token).await
            .map(|opt| opt.as_ref().map(Self::to_domain))
            .map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }

    async fn revoke(&self, token: &str) -> Result<bool, DomainError> {
        self.sea_orm_repo.revoke(token).await
            .map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }

    async fn revoke_all_for_user(&self, user_id: Uuid) -> Result<(), DomainError> {
        self.sea_orm_repo.revoke_all_for_user(user_id).await
            .map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }
}
