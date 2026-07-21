use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::CreateModelCommand::CreateModelCommand;
use crate::dto::ModelResponse::ModelResponse;
use crate::dto::UpdateModelCommand::UpdateModelCommand;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait ModelPort: Send + Sync {
    async fn create_model(&self, command: CreateModelCommand) -> Result<ModelResponse, DomainError>;
    async fn find_model_by_id(&self, id: Uuid) -> Result<ModelResponse, DomainError>;
    /// Returns models scoped to `tenant_id` plus platform-wide (tenant_id = NULL) models.
    async fn find_models_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<ModelResponse>, DomainError>;
    async fn update_model(&self, id: Uuid, command: UpdateModelCommand) -> Result<ModelResponse, DomainError>;
    async fn delete_model(&self, id: Uuid) -> Result<(), DomainError>;
}
