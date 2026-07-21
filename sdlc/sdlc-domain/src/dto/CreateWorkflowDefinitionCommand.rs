use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkflowDefinitionCommand {
    pub tenant_id: Uuid,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    /// Raw JSON text: `{ "nodes": [...], "edges": [...] }` — see `WorkflowGraph`.
    pub definition: String,
}
