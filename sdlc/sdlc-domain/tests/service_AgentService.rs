use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;
use sdlc_domain::dto::Agent::{Agent, NewAgent};
use sdlc_domain::dto::AgentResponse::AgentResponse;
use sdlc_domain::dto::CreateAgentCommand::CreateAgentCommand;
use sdlc_domain::dto::UpdateAgentCommand::UpdateAgentCommand;
use sdlc_domain::port::input::AgentPort::AgentPort;
use sdlc_domain::port::output::AgentRepositoryPort::AgentRepositoryPort;
use sdlc_domain::port::output::SkillRepositoryPort::SkillRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::AgentService::AgentService;

use sdlc_domain::dto::AgentType::AgentType;
use sdlc_domain::dto::Skill::Skill;
use std::sync::Mutex;

#[derive(Default)]
struct MockAgentRepository {
    agents: Mutex<Vec<Agent>>,
}

#[async_trait]
impl AgentRepositoryPort for MockAgentRepository {
    async fn save(&self, agent: &Agent) -> Result<(), DomainError> {
        self.agents.lock().unwrap().push(agent.clone());
        Ok(())
    }

    async fn update(&self, agent: &Agent) -> Result<(), DomainError> {
        let mut agents = self.agents.lock().unwrap();
        if let Some(existing) = agents.iter_mut().find(|a| a.id == agent.id) {
            *existing = agent.clone();
        }
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Agent>, DomainError> {
        Ok(self.agents.lock().unwrap().iter().find(|a| a.id == id).cloned())
    }

    async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Agent>, DomainError> {
        Ok(self.agents.lock().unwrap().iter()
            .filter(|a| a.tenant_id == Some(tenant_id) || a.tenant_id.is_none())
            .cloned()
            .collect())
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        let mut agents = self.agents.lock().unwrap();
        let len_before = agents.len();
        agents.retain(|a| a.id != id);
        Ok(agents.len() != len_before)
    }
}

#[derive(Default)]
struct MockSkillRepository {
    agent_skills: Mutex<Vec<(Uuid, Uuid)>>,
}

#[async_trait]
impl SkillRepositoryPort for MockSkillRepository {
    async fn save(&self, _skill: &Skill) -> Result<(), DomainError> { Ok(()) }
    async fn update(&self, _skill: &Skill) -> Result<(), DomainError> { Ok(()) }
    async fn find_by_id(&self, _id: Uuid) -> Result<Option<Skill>, DomainError> { Ok(None) }
    async fn find_by_tenant_including_global(&self, _tenant_id: Uuid) -> Result<Vec<Skill>, DomainError> { Ok(vec![]) }
    async fn delete_by_id(&self, _id: Uuid) -> Result<bool, DomainError> { Ok(false) }

    async fn set_skills_for_agent(&self, agent_id: Uuid, skill_ids: &[Uuid]) -> Result<(), DomainError> {
        let mut agent_skills = self.agent_skills.lock().unwrap();
        agent_skills.retain(|(a, _)| *a != agent_id);
        agent_skills.extend(skill_ids.iter().map(|s| (agent_id, *s)));
        Ok(())
    }

    async fn find_skill_ids_by_agent(&self, agent_id: Uuid) -> Result<Vec<Uuid>, DomainError> {
        Ok(self.agent_skills.lock().unwrap().iter().filter(|(a, _)| *a == agent_id).map(|(_, s)| *s).collect())
    }

    async fn find_active_skills_by_agent(&self, _agent_id: Uuid) -> Result<Vec<Skill>, DomainError> { Ok(vec![]) }
}

fn service() -> AgentService {
    AgentService::new(Arc::new(MockAgentRepository::default()), Arc::new(MockSkillRepository::default()))
}

fn sample_command(tenant_id: Option<Uuid>) -> CreateAgentCommand {
    CreateAgentCommand {
        tenant_id,
        name: "Ticket Planner".into(),
        agent_type: AgentType::Planner,
        system_prompt: "You are a senior engineer producing implementation plans.".into(),
        provider: "echo".into(),
        model: "echo-1".into(),
        temperature: Some(0.2),
        skill_ids: vec![],
    }
}

#[tokio::test]
async fn create_agent_fails_when_system_prompt_is_empty() {
    let service = service();
    let mut command = sample_command(None);
    command.system_prompt = "".into();
    let result = service.create_agent(command).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn find_agents_by_tenant_includes_global_agents() {
    let service = service();
    let tenant_id = Uuid::new_v4();
    service.create_agent(sample_command(None)).await.unwrap();
    service.create_agent(sample_command(Some(tenant_id))).await.unwrap();
    service.create_agent(sample_command(Some(Uuid::new_v4()))).await.unwrap();

    let results = service.find_agents_by_tenant(tenant_id).await.unwrap();
    assert_eq!(results.len(), 2);
}

#[tokio::test]
async fn update_agent_applies_partial_changes() {
    let service = service();
    let created = service.create_agent(sample_command(None)).await.unwrap();
    let updated = service.update_agent(created.id, UpdateAgentCommand {
        name: None,
        system_prompt: None,
        provider: None,
        model: Some("echo-2".into()),
        temperature: None,
        is_active: Some(false),
        skill_ids: None,
    }).await.unwrap();
    assert_eq!(updated.model, "echo-2");
    assert!(!updated.is_active);
}

#[tokio::test]
async fn create_agent_persists_skill_ids() {
    let service = service();
    let mut command = sample_command(None);
    let skill_id = Uuid::new_v4();
    command.skill_ids = vec![skill_id];
    let created = service.create_agent(command).await.unwrap();
    assert_eq!(created.skill_ids, vec![skill_id]);

    let fetched = service.find_agent_by_id(created.id).await.unwrap();
    assert_eq!(fetched.skill_ids, vec![skill_id]);
}

#[tokio::test]
async fn update_agent_replaces_skill_ids_only_when_provided() {
    let service = service();
    let mut command = sample_command(None);
    command.skill_ids = vec![Uuid::new_v4()];
    let created = service.create_agent(command).await.unwrap();

    let unchanged = service.update_agent(created.id, UpdateAgentCommand {
        name: None, system_prompt: None, provider: None, model: None, temperature: None, is_active: None,
        skill_ids: None,
    }).await.unwrap();
    assert_eq!(unchanged.skill_ids, created.skill_ids);

    let new_skill_id = Uuid::new_v4();
    let replaced = service.update_agent(created.id, UpdateAgentCommand {
        name: None, system_prompt: None, provider: None, model: None, temperature: None, is_active: None,
        skill_ids: Some(vec![new_skill_id]),
    }).await.unwrap();
    assert_eq!(replaced.skill_ids, vec![new_skill_id]);
}

#[tokio::test]
async fn delete_agent_fails_when_not_found() {
    let service = service();
    let result = service.delete_agent(Uuid::new_v4()).await;
    assert!(matches!(result, Err(DomainError::NotFound(_))));
}
