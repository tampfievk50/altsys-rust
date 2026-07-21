use async_trait::async_trait;

use crate::dto::SdlcRun::SdlcRun;
use crate::dto::SdlcStepExecution::SdlcStepExecution;
use crate::r#enum::DomainError::DomainError;

/// Persists a step execution and its owning run together as one atomic unit.
/// The orchestrator advances both after every pipeline step (step outcome +
/// run context/current_step), and a partial write would leave the run and its
/// step history disagreeing about what actually happened.
#[async_trait]
pub trait SdlcCheckpointRepositoryPort: Send + Sync {
    async fn save_checkpoint(&self, step: &SdlcStepExecution, run: &SdlcRun) -> Result<(), DomainError>;
}