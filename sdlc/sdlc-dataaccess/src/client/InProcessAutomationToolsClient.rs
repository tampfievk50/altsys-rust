use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use sdlc_domain::dto::ExecuteToolCommand::ExecuteToolCommand;
use sdlc_domain::port::input::ToolExecutionPort::ToolExecutionPort;
use sdlc_domain::port::output::AutomationToolsClientPort::AutomationToolsClientPort;
use sdlc_domain::r#enum::DomainError::DomainError;

/// Replaces the old HTTP call to the Tools service (automation engine side)
/// with a direct call to the merged `ToolExecutionService`.
pub struct InProcessAutomationToolsClient {
    tool_execution_port: Arc<dyn ToolExecutionPort>,
}

impl InProcessAutomationToolsClient {
    pub fn new(tool_execution_port: Arc<dyn ToolExecutionPort>) -> Self {
        Self { tool_execution_port }
    }
}

#[async_trait]
impl AutomationToolsClientPort for InProcessAutomationToolsClient {
    async fn execute_tool(&self, tool_id: Uuid, action: String, parameters: HashMap<String, String>) -> Result<serde_json::Value, DomainError> {
        let result = self.tool_execution_port
            .execute_tool(tool_id, ExecuteToolCommand { action, parameters, working_directory: None })
            .await?;
        serde_json::to_value(&result).map_err(|e| DomainError::InternalError(format!("Failed to serialize tool execution result: {}", e)))
    }
}
