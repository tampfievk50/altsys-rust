use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use tracing::{error, info};
use uuid::Uuid;
use sdlc_domain::dto::AgentExecution::{AgentExecution, NewAgentExecution};
use sdlc_domain::dto::AgentExecutionResponse::AgentExecutionResponse;
use sdlc_domain::dto::AgentExecutionStatus::AgentExecutionStatus;
use sdlc_domain::dto::ExecuteAgentCommand::ExecuteAgentCommand;
use sdlc_domain::dto::LlmCompletionRequest::LlmCompletionRequest;
use sdlc_domain::dto::Skill::{NewSkill, Skill};
use sdlc_domain::port::input::AgentExecutionPort::AgentExecutionPort;
use sdlc_domain::port::output::AgentExecutionRepositoryPort::AgentExecutionRepositoryPort;
use sdlc_domain::port::output::AgentRepositoryPort::AgentRepositoryPort;
use sdlc_domain::port::output::LlmClientPort::LlmClientPort;
use sdlc_domain::port::output::SkillRepositoryPort::SkillRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::AgentExecutionService::AgentExecutionService;

use sdlc_domain::dto::Agent::{Agent, NewAgent};
use sdlc_domain::dto::AgentType::AgentType;
use sdlc_domain::dto::LlmCompletionResponse::LlmCompletionResponse;
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
    async fn update(&self, _agent: &Agent) -> Result<(), DomainError> {
        Ok(())
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Agent>, DomainError> {
        Ok(self.agents.lock().unwrap().iter().find(|a| a.id == id).cloned())
    }
    async fn find_by_tenant_including_global(&self, _tenant_id: Uuid) -> Result<Vec<Agent>, DomainError> {
        Ok(self.agents.lock().unwrap().clone())
    }
    async fn delete_by_id(&self, _id: Uuid) -> Result<bool, DomainError> {
        Ok(false)
    }
}

#[derive(Default)]
struct MockExecutionRepository {
    executions: Mutex<Vec<AgentExecution>>,
}

#[async_trait]
impl AgentExecutionRepositoryPort for MockExecutionRepository {
    async fn save(&self, execution: &AgentExecution) -> Result<(), DomainError> {
        self.executions.lock().unwrap().push(execution.clone());
        Ok(())
    }
    async fn update(&self, execution: &AgentExecution) -> Result<(), DomainError> {
        let mut executions = self.executions.lock().unwrap();
        if let Some(existing) = executions.iter_mut().find(|e| e.id == execution.id) {
            *existing = execution.clone();
        }
        Ok(())
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AgentExecution>, DomainError> {
        Ok(self.executions.lock().unwrap().iter().find(|e| e.id == id).cloned())
    }
    async fn find_by_agent_id(&self, agent_id: Uuid) -> Result<Vec<AgentExecution>, DomainError> {
        Ok(self.executions.lock().unwrap().iter().filter(|e| e.agent_id == agent_id).cloned().collect())
    }
}

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
    async fn update(&self, _skill: &Skill) -> Result<(), DomainError> { Ok(()) }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Skill>, DomainError> {
        Ok(self.skills.lock().unwrap().iter().find(|s| s.id == id).cloned())
    }
    async fn find_by_tenant_including_global(&self, _tenant_id: Uuid) -> Result<Vec<Skill>, DomainError> {
        Ok(self.skills.lock().unwrap().clone())
    }
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
    async fn find_active_skills_by_agent(&self, agent_id: Uuid) -> Result<Vec<Skill>, DomainError> {
        let skill_ids = self.find_skill_ids_by_agent(agent_id).await?;
        Ok(self.skills.lock().unwrap().iter().filter(|s| skill_ids.contains(&s.id) && s.is_active).cloned().collect())
    }
}

struct EchoClient;
#[async_trait]
impl LlmClientPort for EchoClient {
    fn provider_name(&self) -> &'static str { "echo" }
    async fn complete(&self, request: LlmCompletionRequest) -> Result<LlmCompletionResponse, DomainError> {
        Ok(LlmCompletionResponse { text: format!("echo: {}", request.user_prompt) })
    }
}

/// Echoes the *system* prompt instead of the user prompt, so tests can
/// assert on what skill content actually reached the model.
struct SystemPromptEchoClient;
#[async_trait]
impl LlmClientPort for SystemPromptEchoClient {
    fn provider_name(&self) -> &'static str { "echo" }
    async fn complete(&self, request: LlmCompletionRequest) -> Result<LlmCompletionResponse, DomainError> {
        Ok(LlmCompletionResponse { text: request.system_prompt })
    }
}

struct FailingClient;
#[async_trait]
impl LlmClientPort for FailingClient {
    fn provider_name(&self) -> &'static str { "failing" }
    async fn complete(&self, _request: LlmCompletionRequest) -> Result<LlmCompletionResponse, DomainError> {
        Err(DomainError::InternalError("provider unreachable".into()))
    }
}

fn agent_with_provider(provider: &str, is_active: bool) -> Agent {
    let mut agent = Agent::new(NewAgent {
        tenant_id: None,
        name: "Test Agent".into(),
        agent_type: AgentType::Planner,
        system_prompt: "You plan things.".into(),
        provider: provider.into(),
        model: "test-model".into(),
        temperature: None,
    });
    agent.is_active = is_active;
    agent
}

fn fixture(provider_agent: Agent, clients: Vec<Arc<dyn LlmClientPort>>) -> (AgentExecutionService, Uuid) {
    fixture_with_skills(provider_agent, clients, vec![])
}

fn fixture_with_skills(provider_agent: Agent, clients: Vec<Arc<dyn LlmClientPort>>, skills: Vec<Skill>) -> (AgentExecutionService, Uuid) {
    let agent_repo = Arc::new(MockAgentRepository::default());
    let agent_id = provider_agent.id;
    agent_repo.agents.lock().unwrap().push(provider_agent);
    let execution_repo = Arc::new(MockExecutionRepository::default());
    let skill_repo = Arc::new(MockSkillRepository::default());
    let skill_ids: Vec<Uuid> = skills.iter().map(|s| s.id).collect();
    skill_repo.skills.lock().unwrap().extend(skills);
    skill_repo.agent_skills.lock().unwrap().extend(skill_ids.into_iter().map(|s| (agent_id, s)));
    (AgentExecutionService::new(agent_repo, execution_repo, skill_repo, clients), agent_id)
}

#[tokio::test]
async fn execute_agent_dispatches_to_matching_provider() {
    let (service, agent_id) = fixture(agent_with_provider("echo", true), vec![Arc::new(EchoClient)]);
    let response = service.execute_agent(agent_id, ExecuteAgentCommand {
        tenant_id: Uuid::new_v4(),
        input: "plan the release".into(),
    }).await.unwrap();
    assert_eq!(response.status, "succeeded");
    assert_eq!(response.output.unwrap(), "echo: plan the release");
}

#[tokio::test]
async fn execute_agent_records_failure_when_the_llm_call_fails() {
    let (service, agent_id) = fixture(agent_with_provider("failing", true), vec![Arc::new(FailingClient)]);
    let response = service.execute_agent(agent_id, ExecuteAgentCommand {
        tenant_id: Uuid::new_v4(),
        input: "plan the release".into(),
    }).await.unwrap();
    assert_eq!(response.status, "failed");
    assert!(response.error.is_some());
}

#[tokio::test]
async fn execute_agent_fails_when_disabled() {
    let (service, agent_id) = fixture(agent_with_provider("echo", false), vec![Arc::new(EchoClient)]);
    let result = service.execute_agent(agent_id, ExecuteAgentCommand {
        tenant_id: Uuid::new_v4(),
        input: "plan the release".into(),
    }).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn execute_agent_fails_when_no_client_registered_for_provider() {
    let (service, agent_id) = fixture(agent_with_provider("anthropic", true), vec![Arc::new(EchoClient)]);
    let result = service.execute_agent(agent_id, ExecuteAgentCommand {
        tenant_id: Uuid::new_v4(),
        input: "plan the release".into(),
    }).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn execute_agent_fails_when_input_is_empty() {
    let (service, agent_id) = fixture(agent_with_provider("echo", true), vec![Arc::new(EchoClient)]);
    let result = service.execute_agent(agent_id, ExecuteAgentCommand {
        tenant_id: Uuid::new_v4(),
        input: "".into(),
    }).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn execute_agent_folds_active_attached_skills_into_the_system_prompt() {
    let skill = Skill::new(NewSkill {
        tenant_id: None,
        name: "Rust Idioms".into(),
        description: "Use when writing Rust.".into(),
        content: "Prefer `?` over `.unwrap()`.".into(),
    });
    let (service, agent_id) = fixture_with_skills(
        agent_with_provider("echo", true),
        vec![Arc::new(SystemPromptEchoClient)],
        vec![skill],
    );
    let response = service.execute_agent(agent_id, ExecuteAgentCommand {
        tenant_id: Uuid::new_v4(),
        input: "plan the release".into(),
    }).await.unwrap();
    let prompt = response.output.unwrap();
    assert!(prompt.contains("You plan things."));
    assert!(prompt.contains("Rust Idioms"));
    assert!(prompt.contains("Prefer `?` over `.unwrap()`."));
}

#[tokio::test]
async fn execute_agent_ignores_inactive_skills() {
    let mut skill = Skill::new(NewSkill {
        tenant_id: None,
        name: "Disabled Skill".into(),
        description: "Should not apply.".into(),
        content: "SECRET_MARKER_SHOULD_NOT_APPEAR".into(),
    });
    skill.is_active = false;
    let (service, agent_id) = fixture_with_skills(
        agent_with_provider("echo", true),
        vec![Arc::new(SystemPromptEchoClient)],
        vec![skill],
    );
    let response = service.execute_agent(agent_id, ExecuteAgentCommand {
        tenant_id: Uuid::new_v4(),
        input: "plan the release".into(),
    }).await.unwrap();
    assert!(!response.output.unwrap().contains("SECRET_MARKER_SHOULD_NOT_APPEAR"));
}
