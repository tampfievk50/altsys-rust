use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::CreateSkillCommand::CreateSkillCommand;
use crate::dto::SkillResponse::SkillResponse;
use crate::dto::UpdateSkillCommand::UpdateSkillCommand;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait SkillPort: Send + Sync {
    async fn create_skill(&self, command: CreateSkillCommand) -> Result<SkillResponse, DomainError>;
    async fn find_skill_by_id(&self, id: Uuid) -> Result<SkillResponse, DomainError>;
    /// Returns skills scoped to `tenant_id` plus platform-wide (tenant_id = NULL) skills.
    async fn find_skills_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<SkillResponse>, DomainError>;
    async fn update_skill(&self, id: Uuid, command: UpdateSkillCommand) -> Result<SkillResponse, DomainError>;
    async fn delete_skill(&self, id: Uuid) -> Result<(), DomainError>;
}
