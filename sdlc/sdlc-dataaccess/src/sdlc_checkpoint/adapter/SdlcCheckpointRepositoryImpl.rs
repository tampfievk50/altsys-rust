use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, TransactionTrait};
use tracing::error;

use sdlc_domain::dto::SdlcRun::SdlcRun;
use sdlc_domain::dto::SdlcStepExecution::SdlcStepExecution;
use sdlc_domain::port::output::SdlcCheckpointRepositoryPort::SdlcCheckpointRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::sdlc_run::mapper::SdlcRunDataMapper::SdlcRunDataMapper;
use crate::sdlc_step_execution::mapper::SdlcStepExecutionDataMapper::SdlcStepExecutionDataMapper;

pub struct SdlcCheckpointRepositoryImpl {
    db: DatabaseConnection,
}

impl SdlcCheckpointRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SdlcCheckpointRepositoryPort for SdlcCheckpointRepositoryImpl {
    async fn save_checkpoint(&self, step: &SdlcStepExecution, run: &SdlcRun) -> Result<(), DomainError> {
        let step_model = SdlcStepExecutionDataMapper::to_active_model(step);
        let run_model = SdlcRunDataMapper::to_active_model(run);

        self.db
            .transaction::<_, (), sea_orm::DbErr>(|txn| {
                Box::pin(async move {
                    step_model.update(txn).await?;
                    run_model.update(txn).await?;
                    Ok(())
                })
            })
            .await
            .map_err(|e| {
                error!(error = %e, "Failed to save SDLC checkpoint");
                DomainError::InternalError(e.to_string())
            })
    }
}
