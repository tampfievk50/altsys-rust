use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InstantiateTemplateResponse {
    /// Resolved `WorkflowGraph` JSON text, ready to submit to the Workflow
    /// service's `create_workflow_definition` endpoint.
    pub definition: String,
}
