use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;
use uuid::Uuid;

use sdlc_domain::dto::WorkflowGraph::WorkflowNode;
use sdlc_domain::port::output::AutomationToolsClientPort::AutomationToolsClientPort;
use sdlc_domain::port::output::NodeExecutorPort::NodeExecutorPort;
use sdlc_domain::r#enum::DomainError::DomainError;

/// `executor: "tool"` — runs one registered Tool action. `node.config` must be
/// `{"tool_id": "<uuid>", "action": "<name>", "parameters": {"k": "v", ...}}`
/// (`parameters` optional, string values only). The tool's JSON result lands in
/// the execution context under `"<node.id>_output"`.
pub struct ToolNodeExecutor {
    tools_client: Arc<dyn AutomationToolsClientPort>,
}

impl ToolNodeExecutor {
    pub fn new(tools_client: Arc<dyn AutomationToolsClientPort>) -> Self {
        Self { tools_client }
    }
}

#[async_trait]
impl NodeExecutorPort for ToolNodeExecutor {
    fn executor_name(&self) -> &'static str {
        "tool"
    }

    async fn execute(&self, node: &WorkflowNode, _tenant_id: Uuid, _context: &serde_json::Value) -> Result<serde_json::Value, DomainError> {
        let config = node.config.as_ref()
            .ok_or_else(|| DomainError::ValidationError(format!("Node '{}' (executor: tool) requires config", node.id)))?;
        let tool_id: Uuid = config.get("tool_id")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| DomainError::ValidationError(format!("Node '{}' config must include a valid 'tool_id'", node.id)))?;
        let action = config.get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| DomainError::ValidationError(format!("Node '{}' config must include an 'action'", node.id)))?
            .to_string();
        let parameters: HashMap<String, String> = config.get("parameters")
            .and_then(|v| v.as_object())
            .map(|obj| obj.iter().filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string()))).collect())
            .unwrap_or_default();

        info!(node_id = %node.id, %tool_id, %action, "Executing tool task node");
        let output = self.tools_client.execute_tool(tool_id, action, parameters).await?;

        Ok(serde_json::json!({ format!("{}_output", node.id): output }))
    }
}
