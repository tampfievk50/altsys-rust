use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::CreatePromptCommand::CreatePromptCommand;
use crate::dto::PromptResponse::PromptResponse;
use crate::dto::UpdatePromptCommand::UpdatePromptCommand;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait PromptPort: Send + Sync {
    /// Creates a new prompt version for `command.key`; the version number is
    /// computed automatically (max existing version for the key, tenant + 1).
    async fn create_prompt(&self, command: CreatePromptCommand) -> Result<PromptResponse, DomainError>;
    async fn find_prompt_by_id(&self, id: Uuid) -> Result<PromptResponse, DomainError>;
    async fn find_latest_prompt_by_key(&self, tenant_id: Uuid, key: &str) -> Result<PromptResponse, DomainError>;
    async fn find_prompt_versions_by_key(&self, tenant_id: Uuid, key: &str) -> Result<Vec<PromptResponse>, DomainError>;
    /// Returns the latest version of every distinct key for the tenant.
    async fn find_prompts_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<PromptResponse>, DomainError>;
    async fn update_prompt(&self, id: Uuid, command: UpdatePromptCommand) -> Result<PromptResponse, DomainError>;
    async fn delete_prompt(&self, id: Uuid) -> Result<(), DomainError>;
}
