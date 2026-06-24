use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sso_domain::dto::User::User;
use sso_domain::dto::UserRole::UserRole;
use sso_domain::port::output::UserRepositoryPort::UserRepositoryPort;
use sso_domain::r#enum::DomainError::DomainError;

use crate::user::mapper::UserDataMapper::UserDataMapper;
use crate::user::repository::UserSeaOrmRepository::UserSeaOrmRepository;

pub struct UserRepositoryImpl {
    sea_orm_repo: UserSeaOrmRepository,
}

impl UserRepositoryImpl {
    pub fn new(sea_orm_repo: UserSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl UserRepositoryPort for UserRepositoryImpl {
    async fn save(&self, user: &User) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(UserDataMapper::to_active_model(user)).await
            .map(|_| ()).map_err(|e| { error!(%e, "Failed to save user"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, user: &User) -> Result<(), DomainError> {
        self.sea_orm_repo.update(UserDataMapper::to_active_model(user)).await
            .map(|_| ()).map_err(|e| { error!(%e, "Failed to update user"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(UserDataMapper::to_domain))
            .map_err(|e| { error!(%e, "Failed to find user"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_username_and_tenant(&self, username: &str, tenant_id: Uuid) -> Result<Option<User>, DomainError> {
        self.sea_orm_repo.find_by_username_and_tenant(username, tenant_id).await
            .map(|opt| opt.as_ref().map(UserDataMapper::to_domain))
            .map_err(|e| { error!(%e, "Failed to find user by username"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<User>, DomainError> {
        self.sea_orm_repo.find_by_tenant(tenant_id).await
            .map(|models| models.iter().map(UserDataMapper::to_domain).collect())
            .map_err(|e| { error!(%e, "Failed to list users"); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(%e, "Failed to delete user"); DomainError::InternalError(e.to_string()) })
    }

    async fn save_user_role(&self, ur: &UserRole) -> Result<(), DomainError> {
        self.sea_orm_repo.insert_user_role(ur.user_id, ur.role_id, ur.tenant_id).await
            .map_err(|e| { error!(%e, "Failed to save user role"); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_user_role(&self, user_id: Uuid, role_id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_user_role(user_id, role_id).await
            .map_err(|e| { error!(%e, "Failed to delete user role"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_roles_by_user(&self, user_id: Uuid) -> Result<Vec<UserRole>, DomainError> {
        self.sea_orm_repo.find_roles_by_user(user_id).await
            .map(|models| models.iter().map(|m| UserRole { user_id: m.user_id, role_id: m.role_id, tenant_id: m.tenant_id }).collect())
            .map_err(|e| { error!(%e, "Failed to find user roles"); DomainError::InternalError(e.to_string()) })
    }
}
