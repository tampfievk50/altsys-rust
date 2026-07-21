use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;
use sdlc_domain::dto::CreateSkillCommand::CreateSkillCommand;
use sdlc_domain::dto::Skill::{NewSkill, Skill};
use sdlc_domain::dto::SkillResponse::SkillResponse;
use sdlc_domain::dto::UpdateSkillCommand::UpdateSkillCommand;
use sdlc_domain::port::input::SkillPort::SkillPort;
use sdlc_domain::port::output::SkillRepositoryPort::SkillRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::SkillService::SkillService;

use std::sync::Mutex;

#[derive(Default)]
struct MockSkillRepository {
    skills: Mutex<Vec<Skill>>,
    agent_skills: Mutex<Vec<(Uuid, Uuid)>>,
}

#[async_trait]
impl SkillRepositoryPort for MockSkillRepository {
    async fn save(&self, skill: &Skill) -> Result<(), DomainError> {
        self.skills.lock().unwrap().push(skill.clone());
        Ok(())
    }

    async fn update(&self, skill: &Skill) -> Result<(), DomainError> {
        let mut skills = self.skills.lock().unwrap();
        if let Some(existing) = skills.iter_mut().find(|s| s.id == skill.id) {
            *existing = skill.clone();
        }
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Skill>, DomainError> {
        Ok(self.skills.lock().unwrap().iter().find(|s| s.id == id).cloned())
    }

    async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Skill>, DomainError> {
        Ok(self.skills.lock().unwrap().iter()
            .filter(|s| s.tenant_id == Some(tenant_id) || s.tenant_id.is_none())
            .cloned()
            .collect())
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        let mut skills = self.skills.lock().unwrap();
        let len_before = skills.len();
        skills.retain(|s| s.id != id);
        Ok(skills.len() != len_before)
    }

    async fn set_skills_for_agent(&self, agent_id: Uuid, skill_ids: &[Uuid]) -> Result<(), DomainError> {
        let mut agent_skills = self.agent_skills.lock().unwrap();
        agent_skills.retain(|(a, _)| *a != agent_id);
        agent_skills.extend(skill_ids.iter().map(|s| (agent_id, *s)));
        Ok(())
    }

    async fn find_skill_ids_by_agent(&self, agent_id: Uuid) -> Result<Vec<Uuid>, DomainError> {
        Ok(self.agent_skills.lock().unwrap().iter().filter(|(a, _)| *a == agent_id).map(|(_, s)| *s).collect())
    }

    async fn find_active_skills_by_agent(&self, agent_id: Uuid) -> Result<Vec<Skill>, DomainError> {
        let skill_ids = self.find_skill_ids_by_agent(agent_id).await?;
        Ok(self.skills.lock().unwrap().iter().filter(|s| skill_ids.contains(&s.id) && s.is_active).cloned().collect())
    }
}

fn sample_command(tenant_id: Option<Uuid>) -> CreateSkillCommand {
    CreateSkillCommand {
        tenant_id,
        name: "Rust Idioms".into(),
        description: "Use when writing or reviewing Rust code.".into(),
        content: "Prefer `?` over `.unwrap()`. Favor iterators over manual loops.".into(),
    }
}

#[tokio::test]
async fn create_skill_fails_when_content_is_empty() {
    let service = SkillService::new(Arc::new(MockSkillRepository::default()));
    let mut command = sample_command(None);
    command.content = "".into();
    let result = service.create_skill(command).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn find_skills_by_tenant_includes_global_skills() {
    let service = SkillService::new(Arc::new(MockSkillRepository::default()));
    let tenant_id = Uuid::new_v4();
    service.create_skill(sample_command(None)).await.unwrap();
    service.create_skill(sample_command(Some(tenant_id))).await.unwrap();
    service.create_skill(sample_command(Some(Uuid::new_v4()))).await.unwrap();

    let results = service.find_skills_by_tenant(tenant_id).await.unwrap();
    assert_eq!(results.len(), 2);
}

#[tokio::test]
async fn update_skill_applies_partial_changes() {
    let service = SkillService::new(Arc::new(MockSkillRepository::default()));
    let created = service.create_skill(sample_command(None)).await.unwrap();
    let updated = service.update_skill(created.id, UpdateSkillCommand {
        name: None,
        description: None,
        content: Some("Updated content".into()),
        is_active: Some(false),
    }).await.unwrap();
    assert_eq!(updated.content, "Updated content");
    assert!(!updated.is_active);
}

#[tokio::test]
async fn delete_skill_fails_when_not_found() {
    let service = SkillService::new(Arc::new(MockSkillRepository::default()));
    let result = service.delete_skill(Uuid::new_v4()).await;
    assert!(matches!(result, Err(DomainError::NotFound(_))));
}
