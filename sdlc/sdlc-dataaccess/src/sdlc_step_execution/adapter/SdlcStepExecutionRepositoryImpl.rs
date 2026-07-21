use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::SdlcStepExecution::SdlcStepExecution;
use sdlc_domain::port::output::SdlcStepExecutionRepositoryPort::SdlcStepExecutionRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::sdlc_step_execution::mapper::SdlcStepExecutionDataMapper::SdlcStepExecutionDataMapper;
use crate::sdlc_step_execution::repository::SdlcStepExecutionSeaOrmRepository::SdlcStepExecutionSeaOrmRepository;

pub struct SdlcStepExecutionRepositoryImpl {
    sea_orm_repo: SdlcStepExecutionSeaOrmRepository,
}

impl SdlcStepExecutionRepositoryImpl {
    pub fn new(sea_orm_repo: SdlcStepExecutionSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl SdlcStepExecutionRepositoryPort for SdlcStepExecutionRepositoryImpl {
    async fn save(&self, step: &SdlcStepExecution) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(SdlcStepExecutionDataMapper::to_active_model(step)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save SDLC step execution"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, step: &SdlcStepExecution) -> Result<(), DomainError> {
        self.sea_orm_repo.update(SdlcStepExecutionDataMapper::to_active_model(step)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update SDLC step execution"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_run_id(&self, run_id: Uuid) -> Result<Vec<SdlcStepExecution>, DomainError> {
        self.sea_orm_repo.find_by_run_id(run_id).await
            .map(|models| models.iter().map(SdlcStepExecutionDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list SDLC step executions"); DomainError::InternalError(e.to_string()) })
    }
}
