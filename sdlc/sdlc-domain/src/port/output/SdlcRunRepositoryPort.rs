use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::SdlcRun::SdlcRun;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait SdlcRunRepositoryPort: Send + Sync {
    async fn save(&self, run: &SdlcRun) -> Result<(), DomainError>;
    async fn update(&self, run: &SdlcRun) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SdlcRun>, DomainError>;
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<SdlcRun>, DomainError>;
}
