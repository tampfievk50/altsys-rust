use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAutomationRuleCommand {
    pub tenant_id: Uuid,
    pub name: String,
    pub event_type: String,
    pub match_criteria: Option<String>,
    /// Raw JSON text of an `ActionSpec`.
    pub action: String,
}
