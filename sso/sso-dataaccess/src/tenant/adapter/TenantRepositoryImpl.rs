use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sso_domain::dto::Tenant::Tenant;
use sso_domain::port::output::TenantRepositoryPort::TenantRepositoryPort;
use sso_domain::r#enum::DomainError::DomainError;

use crate::tenant::mapper::TenantDataMapper::TenantDataMapper;
use crate::tenant::repository::TenantSeaOrmRepository::TenantSeaOrmRepository;

pub struct TenantRepositoryImpl {
    sea_orm_repo: TenantSeaOrmRepository,
}

impl TenantRepositoryImpl {
    pub fn new(sea_orm_repo: TenantSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl TenantRepositoryPort for TenantRepositoryImpl {
    async fn save(&self, tenant: &Tenant) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(TenantDataMapper::to_active_model(tenant)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save tenant"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, tenant: &Tenant) -> Result<(), DomainError> {
        self.sea_orm_repo.update(TenantDataMapper::to_active_model(tenant)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update tenant"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tenant>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(TenantDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find tenant"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<Tenant>, DomainError> {
        self.sea_orm_repo.find_by_slug(slug).await
            .map(|opt| opt.as_ref().map(TenantDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find tenant by slug"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_all(&self) -> Result<Vec<Tenant>, DomainError> {
        self.sea_orm_repo.find_all().await
            .map(|models| models.iter().map(TenantDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list tenants"); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(error = %e, "Failed to delete tenant"); DomainError::InternalError(e.to_string()) })
    }
}
