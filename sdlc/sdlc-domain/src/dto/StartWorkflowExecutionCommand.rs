use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartWorkflowExecutionCommand {
    pub tenant_id: Uuid,
    pub workflow_definition_id: Uuid,
    /// Raw JSON object text seeding the execution context. Defaults to `{}`.
    pub context: Option<String>,
}
