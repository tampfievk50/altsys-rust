use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use sdlc_domain::dto::ExecuteAgentCommand::ExecuteAgentCommand;
use sdlc_domain::port::input::AgentExecutionPort::AgentExecutionPort;
use sdlc_domain::port::output::AgentsClientPort::AgentsClientPort;
use sdlc_domain::r#enum::DomainError::DomainError;

/// Replaces the old HTTP call to the Agents service with a direct call to
/// the merged `AgentExecutionService`.
pub struct InProcessAgentsClient {
    agent_execution_port: Arc<dyn AgentExecutionPort>,
}

impl InProcessAgentsClient {
    pub fn new(agent_execution_port: Arc<dyn AgentExecutionPort>) -> Self {
        Self { agent_execution_port }
    }
}

#[async_trait]
impl AgentsClientPort for InProcessAgentsClient {
    async fn execute_agent(&self, agent_id: Uuid, tenant_id: Uuid, input: String) -> Result<String, DomainError> {
        let execution = self.agent_execution_port
            .execute_agent(agent_id, ExecuteAgentCommand { tenant_id, input })
            .await?;

        if execution.status == "succeeded" {
            Ok(execution.output.unwrap_or_default())
        }
        else {
            Err(DomainError::InternalError(execution.error.unwrap_or_else(|| "Agent execution failed".into())))
        }
    }
}
