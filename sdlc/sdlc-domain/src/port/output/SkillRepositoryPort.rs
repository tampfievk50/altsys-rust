use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::Skill::Skill;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait SkillRepositoryPort: Send + Sync {
    async fn save(&self, skill: &Skill) -> Result<(), DomainError>;
    async fn update(&self, skill: &Skill) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Skill>, DomainError>;
    /// Returns rows where `tenant_id = tenant_id` OR `tenant_id IS NULL` (global skills).
    async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Skill>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;

    /// Full-replace: an agent's attached skills become exactly `skill_ids`.
    async fn set_skills_for_agent(&self, agent_id: Uuid, skill_ids: &[Uuid]) -> Result<(), DomainError>;
    async fn find_skill_ids_by_agent(&self, agent_id: Uuid) -> Result<Vec<Uuid>, DomainError>;
    /// Only the skills an execution should actually apply — active ones attached to the agent.
    async fn find_active_skills_by_agent(&self, agent_id: Uuid) -> Result<Vec<Skill>, DomainError>;
}
