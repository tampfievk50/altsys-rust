use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{error, info};
use uuid::Uuid;

use crate::dto::AgentExecution::{AgentExecution, NewAgentExecution};
use crate::dto::AgentExecutionResponse::AgentExecutionResponse;
use crate::dto::AgentExecutionStatus::AgentExecutionStatus;
use crate::dto::ExecuteAgentCommand::ExecuteAgentCommand;
use crate::dto::LlmCompletionRequest::LlmCompletionRequest;
use crate::dto::Skill::Skill;
use crate::port::input::AgentExecutionPort::AgentExecutionPort;
use crate::port::output::AgentExecutionRepositoryPort::AgentExecutionRepositoryPort;
use crate::port::output::AgentRepositoryPort::AgentRepositoryPort;
use crate::port::output::LlmClientPort::LlmClientPort;
use crate::port::output::SkillRepositoryPort::SkillRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct AgentExecutionService {
    agent_repository: Arc<dyn AgentRepositoryPort>,
    execution_repository: Arc<dyn AgentExecutionRepositoryPort>,
    skill_repository: Arc<dyn SkillRepositoryPort>,
    llm_clients: HashMap<String, Arc<dyn LlmClientPort>>,
}

impl AgentExecutionService {
    pub fn new(
        agent_repository: Arc<dyn AgentRepositoryPort>,
        execution_repository: Arc<dyn AgentExecutionRepositoryPort>,
        skill_repository: Arc<dyn SkillRepositoryPort>,
        llm_clients: Vec<Arc<dyn LlmClientPort>>,
    ) -> Self {
        let llm_clients = llm_clients.into_iter().map(|c| (c.provider_name().to_string(), c)).collect();
        Self { agent_repository, execution_repository, skill_repository, llm_clients }
    }

    /// Folds each attached skill's instructions into the agent's system prompt
    /// under its own heading, so the model sees a clearly-scoped set of
    /// capabilities rather than a wall of merged text. No skills attached
    /// (the common case) leaves the base prompt untouched.
    fn augment_system_prompt(base_system_prompt: &str, skills: &[Skill]) -> String {
        if skills.is_empty() {
            return base_system_prompt.to_string();
        }
        let mut prompt = String::from(base_system_prompt);
        prompt.push_str("\n\n# Skills\n\nYou have access to the following skills. Apply each one when its description matches the current task.\n");
        for skill in skills {
            prompt.push_str(&format!("\n## {}\n{}\n\n{}\n", skill.name, skill.description, skill.content));
        }
        prompt
    }

    fn to_response(execution: &AgentExecution) -> AgentExecutionResponse {
        AgentExecutionResponse {
            id: execution.id,
            tenant_id: execution.tenant_id,
            agent_id: execution.agent_id,
            input: execution.input.clone(),
            output: execution.output.clone(),
            status: execution.status.to_string(),
            error: execution.error.clone(),
            started_at: execution.started_at,
            completed_at: execution.completed_at,
            created_at: execution.created_at,
            updated_at: execution.updated_at,
        }
    }
}

#[async_trait]
impl AgentExecutionPort for AgentExecutionService {
    async fn execute_agent(&self, agent_id: Uuid, command: ExecuteAgentCommand) -> Result<AgentExecutionResponse, DomainError> {
        if command.input.trim().is_empty() {
            return Err(DomainError::ValidationError("Input cannot be empty".into()));
        }
        let agent = self.agent_repository.find_by_id(agent_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Agent not found: {}", agent_id)))?;
        if !agent.is_active {
            return Err(DomainError::ValidationError(format!("Agent '{}' is disabled", agent.name)));
        }
        let client = self.llm_clients.get(&agent.provider)
            .ok_or_else(|| DomainError::ValidationError(format!("No LLM client registered for provider '{}'", agent.provider)))?;
        let skills = self.skill_repository.find_active_skills_by_agent(agent_id).await?;

        let mut execution = AgentExecution::new(NewAgentExecution {
            tenant_id: command.tenant_id,
            agent_id,
            input: command.input.clone(),
        });
        self.execution_repository.save(&execution).await?;

        info!(agent_id = %agent_id, provider = %agent.provider, skill_count = skills.len(), "Executing agent");
        let result = client.complete(LlmCompletionRequest {
            system_prompt: Self::augment_system_prompt(&agent.system_prompt, &skills),
            user_prompt: command.input,
            model: agent.model.clone(),
            temperature: agent.temperature,
        }).await;

        match result {
            Ok(response) => {
                execution.status = AgentExecutionStatus::Succeeded;
                execution.output = Some(response.text);
            }
            Err(e) => {
                error!(agent_id = %agent_id, error = %e, "Agent execution failed");
                execution.status = AgentExecutionStatus::Failed;
                execution.error = Some(e.to_string());
            }
        }
        execution.completed_at = Some(Utc::now());
        execution.updated_at = Utc::now();
        self.execution_repository.update(&execution).await?;

        Ok(Self::to_response(&execution))
    }

    async fn find_execution_by_id(&self, id: Uuid) -> Result<AgentExecutionResponse, DomainError> {
        let execution = self.execution_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Agent execution not found: {}", id)))?;
        Ok(Self::to_response(&execution))
    }

    async fn find_executions_by_agent(&self, agent_id: Uuid) -> Result<Vec<AgentExecutionResponse>, DomainError> {
        let executions = self.execution_repository.find_by_agent_id(agent_id).await?;
        Ok(executions.iter().map(Self::to_response).collect())
    }
}
