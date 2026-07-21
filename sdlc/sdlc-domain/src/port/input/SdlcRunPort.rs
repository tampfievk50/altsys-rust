use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::SdlcRunResponse::SdlcRunResponse;
use crate::dto::SdlcStepExecutionResponse::SdlcStepExecutionResponse;
use crate::dto::StartSdlcRunCommand::StartSdlcRunCommand;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait SdlcRunPort: Send + Sync {
    /// Runs the fixed Autonomous SDLC pipeline (see `SdlcStep`) end to end,
    /// synchronously, returning once the run reaches a terminal state
    /// (`completed` or `failed`).
    async fn start_run(&self, command: StartSdlcRunCommand) -> Result<SdlcRunResponse, DomainError>;
    async fn find_run_by_id(&self, id: Uuid) -> Result<SdlcRunResponse, DomainError>;
    async fn find_runs_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<SdlcRunResponse>, DomainError>;
    /// Checkpoint log: every attempt of every step run so far.
    async fn find_step_executions(&self, run_id: Uuid) -> Result<Vec<SdlcStepExecutionResponse>, DomainError>;
}
