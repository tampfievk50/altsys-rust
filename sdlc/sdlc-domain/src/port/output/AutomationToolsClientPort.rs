use std::collections::HashMap;

use async_trait::async_trait;
use uuid::Uuid;

use crate::r#enum::DomainError::DomainError;

/// Driven port consumed by the automation engine's action dispatcher. Named
/// distinctly from `SdlcToolsClientPort` since the two callers need
/// different shapes (this one returns raw JSON with no working directory).
#[async_trait]
pub trait AutomationToolsClientPort: Send + Sync {
    async fn execute_tool(&self, tool_id: Uuid, action: String, parameters: HashMap<String, String>) -> Result<serde_json::Value, DomainError>;
}
