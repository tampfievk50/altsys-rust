use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::Skill::Skill;
use sdlc_domain::port::output::SkillRepositoryPort::SkillRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::skill::mapper::SkillDataMapper::SkillDataMapper;
use crate::skill::repository::SkillSeaOrmRepository::SkillSeaOrmRepository;

pub struct SkillRepositoryImpl {
    sea_orm_repo: SkillSeaOrmRepository,
}

impl SkillRepositoryImpl {
    pub fn new(sea_orm_repo: SkillSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl SkillRepositoryPort for SkillRepositoryImpl {
    async fn save(&self, skill: &Skill) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(SkillDataMapper::to_active_model(skill)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save skill"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, skill: &Skill) -> Result<(), DomainError> {
        self.sea_orm_repo.update(SkillDataMapper::to_active_model(skill)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update skill"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Skill>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(SkillDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find skill"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Skill>, DomainError> {
        self.sea_orm_repo.find_by_tenant_including_global(tenant_id).await
            .map(|skills| skills.iter().map(SkillDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list skills"); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(error = %e, "Failed to delete skill"); DomainError::InternalError(e.to_string()) })
    }

    async fn set_skills_for_agent(&self, agent_id: Uuid, skill_ids: &[Uuid]) -> Result<(), DomainError> {
        self.sea_orm_repo.set_skills_for_agent(agent_id, skill_ids).await
            .map_err(|e| { error!(error = %e, "Failed to set agent skills"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_skill_ids_by_agent(&self, agent_id: Uuid) -> Result<Vec<Uuid>, DomainError> {
        self.sea_orm_repo.find_skill_ids_by_agent(agent_id).await
            .map_err(|e| { error!(error = %e, "Failed to find agent skill ids"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_active_skills_by_agent(&self, agent_id: Uuid) -> Result<Vec<Skill>, DomainError> {
        self.sea_orm_repo.find_active_skills_by_agent(agent_id).await
            .map(|skills| skills.iter().map(SkillDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to find active agent skills"); DomainError::InternalError(e.to_string()) })
    }
}
