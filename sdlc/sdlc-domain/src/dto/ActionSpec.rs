use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::r#enum::DomainError::DomainError;

/// What an `AutomationRule` does when it fires. Internally tagged on `action_type`
/// so the JSON shape is self-describing, e.g.
/// `{"action_type":"execute_tool","tool_id":"...","action":"build","parameters":{}}`.
/// New action types extend this enum without touching the dispatch mechanism's
/// storage shape — the whole thing round-trips through one `action` TEXT column.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "action_type")]
pub enum ActionSpec {
    /// Forwarded verbatim as the request body to the SDLC service's `start_run`.
    StartSdlcRun { parameters: serde_json::Value },
    StartWorkflowExecution { workflow_definition_id: Uuid, parameters: serde_json::Value },
    ExecuteTool { tool_id: Uuid, action: String, parameters: HashMap<String, String> },
    /// The Plugin SDK dispatch path: invokes a registered `Plugin`'s webhook.
    InvokePlugin { plugin_id: Uuid, parameters: serde_json::Value },
    /// Runs a `Classifier` agent against the event payload, expecting strict JSON
    /// output `{"workflow_key": "...", ...}`; resolves `workflow_key` to the
    /// tenant's latest `WorkflowDefinition` of that key and starts it with the
    /// event payload as context. Falls back to `fallback_action` if the agent's
    /// output can't be parsed or no workflow definition matches the key.
    ClassifyAndDispatch { classifier_agent_id: Uuid, fallback_action: Box<ActionSpec> },
}

impl ActionSpec {
    pub fn parse(raw: &str) -> Result<Self, DomainError> {
        serde_json::from_str(raw).map_err(|e| DomainError::ValidationError(format!("Invalid action JSON: {}", e)))
    }

    pub fn to_json_string(&self) -> Result<String, DomainError> {
        serde_json::to_string(self).map_err(|e| DomainError::InternalError(format!("Failed to serialize action: {}", e)))
    }
}
