use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sso_domain::dto::Role::Role;
use sso_domain::dto::RolePermission::RolePermission;
use sso_domain::port::output::RoleRepositoryPort::RoleRepositoryPort;
use sso_domain::r#enum::DomainError::DomainError;

use crate::role::mapper::RoleDataMapper::RoleDataMapper;
use crate::role::repository::RoleSeaOrmRepository::RoleSeaOrmRepository;

pub struct RoleRepositoryImpl {
    sea_orm_repo: RoleSeaOrmRepository,
}

impl RoleRepositoryImpl {
    pub fn new(sea_orm_repo: RoleSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl RoleRepositoryPort for RoleRepositoryImpl {
    async fn save(&self, role: &Role) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(RoleDataMapper::to_active_model(role)).await
            .map(|_| ()).map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, role: &Role) -> Result<(), DomainError> {
        self.sea_orm_repo.update(RoleDataMapper::to_active_model(role)).await
            .map(|_| ()).map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(RoleDataMapper::to_domain))
            .map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Role>, DomainError> {
        self.sea_orm_repo.find_by_tenant(tenant_id).await
            .map(|models| models.iter().map(RoleDataMapper::to_domain).collect())
            .map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_name_and_tenant(&self, name: &str, tenant_id: Uuid) -> Result<Option<Role>, DomainError> {
        self.sea_orm_repo.find_by_name_and_tenant(name, tenant_id).await
            .map(|opt| opt.as_ref().map(RoleDataMapper::to_domain))
            .map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }

    async fn save_role_permission(&self, rp: &RolePermission) -> Result<(), DomainError> {
        self.sea_orm_repo.insert_role_permission(rp.role_id, rp.permission_id).await
            .map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_role_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_role_permission(role_id, permission_id).await
            .map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }

    async fn find_permissions_by_role(&self, role_id: Uuid) -> Result<Vec<RolePermission>, DomainError> {
        self.sea_orm_repo.find_permissions_by_role(role_id).await
            .map(|models| models.iter().map(|m| RolePermission { role_id: m.role_id, permission_id: m.permission_id }).collect())
            .map_err(|e| { error!(%e); DomainError::InternalError(e.to_string()) })
    }
}
