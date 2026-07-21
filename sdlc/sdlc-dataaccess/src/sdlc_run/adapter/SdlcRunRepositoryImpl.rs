use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::SdlcRun::SdlcRun;
use sdlc_domain::port::output::SdlcRunRepositoryPort::SdlcRunRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::sdlc_run::mapper::SdlcRunDataMapper::SdlcRunDataMapper;
use crate::sdlc_run::repository::SdlcRunSeaOrmRepository::SdlcRunSeaOrmRepository;

pub struct SdlcRunRepositoryImpl {
    sea_orm_repo: SdlcRunSeaOrmRepository,
}

impl SdlcRunRepositoryImpl {
    pub fn new(sea_orm_repo: SdlcRunSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl SdlcRunRepositoryPort for SdlcRunRepositoryImpl {
    async fn save(&self, run: &SdlcRun) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(SdlcRunDataMapper::to_active_model(run)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save SDLC run"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, run: &SdlcRun) -> Result<(), DomainError> {
        self.sea_orm_repo.update(SdlcRunDataMapper::to_active_model(run)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update SDLC run"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<SdlcRun>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(SdlcRunDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find SDLC run"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<SdlcRun>, DomainError> {
        self.sea_orm_repo.find_by_tenant(tenant_id).await
            .map(|models| models.iter().map(SdlcRunDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list SDLC runs"); DomainError::InternalError(e.to_string()) })
    }
}
