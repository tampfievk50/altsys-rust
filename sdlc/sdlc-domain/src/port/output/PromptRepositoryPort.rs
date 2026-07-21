use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::Prompt::Prompt;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait PromptRepositoryPort: Send + Sync {
    async fn save(&self, prompt: &Prompt) -> Result<(), DomainError>;
    async fn update(&self, prompt: &Prompt) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Prompt>, DomainError>;
    async fn find_all_by_key_and_tenant(&self, tenant_id: Uuid, key: &str) -> Result<Vec<Prompt>, DomainError>;
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Prompt>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;
}
