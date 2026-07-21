use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;
use uuid::Uuid;

use sdlc_domain::dto::WorkflowGraph::WorkflowNode;
use sdlc_domain::port::output::AgentsClientPort::AgentsClientPort;
use sdlc_domain::port::output::NodeExecutorPort::NodeExecutorPort;
use sdlc_domain::r#enum::DomainError::DomainError;

/// `executor: "agent"` — runs a configured Agent against the current execution
/// context. `node.config` must be `{"agent_id": "<uuid>", "instruction": "<text>"}`
/// (`instruction` defaults to a generic prompt if omitted). The agent's text output
/// lands in the execution context under `"<node.id>_output"` so downstream nodes
/// and edge conditions can reference it without clobbering other agent/tool nodes'
/// outputs in the same graph.
pub struct AgentNodeExecutor {
    agents_client: Arc<dyn AgentsClientPort>,
}

impl AgentNodeExecutor {
    pub fn new(agents_client: Arc<dyn AgentsClientPort>) -> Self {
        Self { agents_client }
    }
}

#[async_trait]
impl NodeExecutorPort for AgentNodeExecutor {
    fn executor_name(&self) -> &'static str {
        "agent"
    }

    async fn execute(&self, node: &WorkflowNode, tenant_id: Uuid, context: &serde_json::Value) -> Result<serde_json::Value, DomainError> {
        let config = node.config.as_ref()
            .ok_or_else(|| DomainError::ValidationError(format!("Node '{}' (executor: agent) requires config", node.id)))?;
        let agent_id: Uuid = config.get("agent_id")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| DomainError::ValidationError(format!("Node '{}' config must include a valid 'agent_id'", node.id)))?;
        let instruction = config.get("instruction").and_then(|v| v.as_str()).unwrap_or("Process this task.");

        let input = format!("{}\n\nContext:\n{}", instruction, context);
        info!(node_id = %node.id, %agent_id, "Executing agent task node");
        let output = self.agents_client.execute_agent(agent_id, tenant_id, input).await?;

        Ok(serde_json::json!({ format!("{}_output", node.id): output }))
    }
}
