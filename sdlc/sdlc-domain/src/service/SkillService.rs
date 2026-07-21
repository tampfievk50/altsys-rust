use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::CreateSkillCommand::CreateSkillCommand;
use crate::dto::Skill::{NewSkill, Skill};
use crate::dto::SkillResponse::SkillResponse;
use crate::dto::UpdateSkillCommand::UpdateSkillCommand;
use crate::port::input::SkillPort::SkillPort;
use crate::port::output::SkillRepositoryPort::SkillRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct SkillService {
    skill_repository: Arc<dyn SkillRepositoryPort>,
}

impl SkillService {
    pub fn new(skill_repository: Arc<dyn SkillRepositoryPort>) -> Self {
        Self { skill_repository }
    }

    fn to_response(skill: &Skill) -> SkillResponse {
        SkillResponse {
            id: skill.id,
            tenant_id: skill.tenant_id,
            name: skill.name.clone(),
            description: skill.description.clone(),
            content: skill.content.clone(),
            is_active: skill.is_active,
            created_at: skill.created_at,
            updated_at: skill.updated_at,
            created_by: skill.created_by,
            updated_by: skill.updated_by,
        }
    }
}

#[async_trait]
impl SkillPort for SkillService {
    async fn create_skill(&self, command: CreateSkillCommand) -> Result<SkillResponse, DomainError> {
        info!(name = %command.name, "Registering skill");
        if command.name.trim().is_empty() {
            return Err(DomainError::ValidationError("Name cannot be empty".into()));
        }
        if command.content.trim().is_empty() {
            return Err(DomainError::ValidationError("Content cannot be empty".into()));
        }
        let skill = Skill::new(NewSkill {
            tenant_id: command.tenant_id,
            name: command.name,
            description: command.description,
            content: command.content,
        });
        self.skill_repository.save(&skill).await?;
        info!(skill_id = %skill.id, "Skill registered");
        Ok(Self::to_response(&skill))
    }

    async fn find_skill_by_id(&self, id: Uuid) -> Result<SkillResponse, DomainError> {
        let skill = self.skill_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Skill not found: {}", id)))?;
        Ok(Self::to_response(&skill))
    }

    async fn find_skills_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<SkillResponse>, DomainError> {
        let skills = self.skill_repository.find_by_tenant_including_global(tenant_id).await?;
        Ok(skills.iter().map(Self::to_response).collect())
    }

    async fn update_skill(&self, id: Uuid, command: UpdateSkillCommand) -> Result<SkillResponse, DomainError> {
        info!(skill_id = %id, "Updating skill");
        let mut skill = self.skill_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Skill not found: {}", id)))?;
        if let Some(name) = command.name {
            skill.name = name;
        }
        if let Some(description) = command.description {
            skill.description = description;
        }
        if let Some(content) = command.content {
            skill.content = content;
        }
        if let Some(is_active) = command.is_active {
            skill.is_active = is_active;
        }
        skill.updated_at = Utc::now();
        self.skill_repository.update(&skill).await?;
        Ok(Self::to_response(&skill))
    }

    async fn delete_skill(&self, id: Uuid) -> Result<(), DomainError> {
        info!(skill_id = %id, "Deleting skill");
        let deleted = self.skill_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(skill_id = %id, "Skill not found for deletion");
            return Err(DomainError::NotFound(format!("Skill not found: {}", id)));
        }
        Ok(())
    }
}
