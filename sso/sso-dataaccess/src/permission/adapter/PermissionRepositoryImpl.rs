use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sso_domain::dto::Permission::Permission;
use sso_domain::port::output::PermissionRepositoryPort::PermissionRepositoryPort;
use sso_domain::r#enum::DomainError::DomainError;

use crate::permission::mapper::PermissionDataMapper::PermissionDataMapper;
use crate::permission::repository::PermissionSeaOrmRepository::PermissionSeaOrmRepository;

pub struct PermissionRepositoryImpl {
    sea_orm_repo: PermissionSeaOrmRepository,
}

impl PermissionRepositoryImpl {
    pub fn new(sea_orm_repo: PermissionSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl PermissionRepositoryPort for PermissionRepositoryImpl {
    async fn save(&self, p: &Permission) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(PermissionDataMapper::to_active_model(p)).await
            .map(|_| ()).map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, p: &Permission) -> Result<(), DomainError> {
        self.sea_orm_repo.update(PermissionDataMapper::to_active_model(p)).await
            .map(|_| ()).map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Permission>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(PermissionDataMapper::to_domain))
            .map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Permission>, DomainError> {
        self.sea_orm_repo.find_by_name(name).await
            .map(|opt| opt.as_ref().map(PermissionDataMapper::to_domain))
            .map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }

    async fn find_all(&self) -> Result<Vec<Permission>, DomainError> {
        self.sea_orm_repo.find_all().await
            .map(|models| models.iter().map(PermissionDataMapper::to_domain).collect())
            .map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }
}
