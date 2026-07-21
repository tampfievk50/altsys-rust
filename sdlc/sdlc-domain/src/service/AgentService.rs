use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::Agent::{Agent, NewAgent};
use crate::dto::AgentResponse::AgentResponse;
use crate::dto::CreateAgentCommand::CreateAgentCommand;
use crate::dto::UpdateAgentCommand::UpdateAgentCommand;
use crate::port::input::AgentPort::AgentPort;
use crate::port::output::AgentRepositoryPort::AgentRepositoryPort;
use crate::port::output::SkillRepositoryPort::SkillRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct AgentService {
    agent_repository: Arc<dyn AgentRepositoryPort>,
    skill_repository: Arc<dyn SkillRepositoryPort>,
}

impl AgentService {
    pub fn new(agent_repository: Arc<dyn AgentRepositoryPort>, skill_repository: Arc<dyn SkillRepositoryPort>) -> Self {
        Self { agent_repository, skill_repository }
    }

    fn to_response(agent: &Agent, skill_ids: Vec<Uuid>) -> AgentResponse {
        AgentResponse {
            id: agent.id,
            tenant_id: agent.tenant_id,
            name: agent.name.clone(),
            agent_type: agent.agent_type,
            system_prompt: agent.system_prompt.clone(),
            provider: agent.provider.clone(),
            model: agent.model.clone(),
            temperature: agent.temperature,
            is_active: agent.is_active,
            skill_ids,
            created_at: agent.created_at,
            updated_at: agent.updated_at,
            created_by: agent.created_by,
            updated_by: agent.updated_by,
        }
    }
}

#[async_trait]
impl AgentPort for AgentService {
    async fn create_agent(&self, command: CreateAgentCommand) -> Result<AgentResponse, DomainError> {
        info!(name = %command.name, agent_type = %command.agent_type, "Registering agent");
        if command.name.trim().is_empty() {
            return Err(DomainError::ValidationError("Name cannot be empty".into()));
        }
        if command.system_prompt.trim().is_empty() {
            return Err(DomainError::ValidationError("System prompt cannot be empty".into()));
        }
        if command.provider.trim().is_empty() {
            return Err(DomainError::ValidationError("Provider cannot be empty".into()));
        }
        if command.model.trim().is_empty() {
            return Err(DomainError::ValidationError("Model cannot be empty".into()));
        }
        let agent = Agent::new(NewAgent {
            tenant_id: command.tenant_id,
            name: command.name,
            agent_type: command.agent_type,
            system_prompt: command.system_prompt,
            provider: command.provider,
            model: command.model,
            temperature: command.temperature,
        });
        self.agent_repository.save(&agent).await?;
        self.skill_repository.set_skills_for_agent(agent.id, &command.skill_ids).await?;
        info!(agent_id = %agent.id, "Agent registered");
        Ok(Self::to_response(&agent, command.skill_ids))
    }

    async fn find_agent_by_id(&self, id: Uuid) -> Result<AgentResponse, DomainError> {
        let agent = self.agent_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Agent not found: {}", id)))?;
        let skill_ids = self.skill_repository.find_skill_ids_by_agent(id).await?;
        Ok(Self::to_response(&agent, skill_ids))
    }

    async fn find_agents_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<AgentResponse>, DomainError> {
        let agents = self.agent_repository.find_by_tenant_including_global(tenant_id).await?;
        let mut responses = Vec::with_capacity(agents.len());
        for agent in &agents {
            let skill_ids = self.skill_repository.find_skill_ids_by_agent(agent.id).await?;
            responses.push(Self::to_response(agent, skill_ids));
        }
        Ok(responses)
    }

    async fn update_agent(&self, id: Uuid, command: UpdateAgentCommand) -> Result<AgentResponse, DomainError> {
        info!(agent_id = %id, "Updating agent");
        let mut agent = self.agent_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Agent not found: {}", id)))?;
        if let Some(name) = command.name {
            agent.name = name;
        }
        if let Some(system_prompt) = command.system_prompt {
            agent.system_prompt = system_prompt;
        }
        if let Some(provider) = command.provider {
            agent.provider = provider;
        }
        if let Some(model) = command.model {
            agent.model = model;
        }
        if let Some(temperature) = command.temperature {
            agent.temperature = Some(temperature);
        }
        if let Some(is_active) = command.is_active {
            agent.is_active = is_active;
        }
        agent.updated_at = Utc::now();
        self.agent_repository.update(&agent).await?;

        let skill_ids = if let Some(skill_ids) = command.skill_ids {
            self.skill_repository.set_skills_for_agent(id, &skill_ids).await?;
            skill_ids
        }
        else {
            self.skill_repository.find_skill_ids_by_agent(id).await?
        };
        Ok(Self::to_response(&agent, skill_ids))
    }

    async fn delete_agent(&self, id: Uuid) -> Result<(), DomainError> {
        info!(agent_id = %id, "Deleting agent");
        let deleted = self.agent_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(agent_id = %id, "Agent not found for deletion");
            return Err(DomainError::NotFound(format!("Agent not found: {}", id)));
        }
        Ok(())
    }
}
