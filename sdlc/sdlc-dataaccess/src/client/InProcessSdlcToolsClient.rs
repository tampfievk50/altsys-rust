use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use sdlc_domain::dto::ExecuteToolCommand::ExecuteToolCommand;
use sdlc_domain::dto::ToolRunResult::ToolRunResult;
use sdlc_domain::port::input::ToolExecutionPort::ToolExecutionPort;
use sdlc_domain::port::output::SdlcToolsClientPort::SdlcToolsClientPort;
use sdlc_domain::r#enum::DomainError::DomainError;

/// Replaces the old HTTP call to the Tools service (SDLC orchestrator side)
/// with a direct call to the merged `ToolExecutionService`.
pub struct InProcessSdlcToolsClient {
    tool_execution_port: Arc<dyn ToolExecutionPort>,
}

impl InProcessSdlcToolsClient {
    pub fn new(tool_execution_port: Arc<dyn ToolExecutionPort>) -> Self {
        Self { tool_execution_port }
    }
}

#[async_trait]
impl SdlcToolsClientPort for InProcessSdlcToolsClient {
    async fn execute_tool(
        &self,
        tool_id: Uuid,
        action: String,
        parameters: HashMap<String, String>,
        working_directory: Option<String>,
    ) -> Result<ToolRunResult, DomainError> {
        let result = self.tool_execution_port
            .execute_tool(tool_id, ExecuteToolCommand { action, parameters, working_directory })
            .await?;
        Ok(ToolRunResult { success: result.success, output: result.output, error: result.error })
    }
}
