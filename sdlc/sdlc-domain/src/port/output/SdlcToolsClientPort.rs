use std::collections::HashMap;

use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::ToolRunResult::ToolRunResult;
use crate::r#enum::DomainError::DomainError;

/// Driven port consumed by the SDLC orchestrator: runs a registered Git/GitHub/
/// Jira/build-tool action (git branch/commit/push, cargo/maven/gradle build/test,
/// GitHub PR creation, Jira issue fetch/transition). Named distinctly from
/// `AutomationToolsClientPort` since the two callers need different shapes
/// (this one returns the richer `ToolRunResult` and takes a working directory).
#[async_trait]
pub trait SdlcToolsClientPort: Send + Sync {
    async fn execute_tool(
        &self,
        tool_id: Uuid,
        action: String,
        parameters: HashMap<String, String>,
        working_directory: Option<String>,
    ) -> Result<ToolRunResult, DomainError>;
}
