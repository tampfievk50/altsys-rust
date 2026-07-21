use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::SdlcStepExecution::SdlcStepExecution;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait SdlcStepExecutionRepositoryPort: Send + Sync {
    async fn save(&self, step: &SdlcStepExecution) -> Result<(), DomainError>;
    async fn update(&self, step: &SdlcStepExecution) -> Result<(), DomainError>;
    async fn find_by_run_id(&self, run_id: Uuid) -> Result<Vec<SdlcStepExecution>, DomainError>;
}
