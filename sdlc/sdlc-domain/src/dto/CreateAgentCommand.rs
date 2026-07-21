use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dto::AgentType::AgentType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentCommand {
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub agent_type: AgentType,
    pub system_prompt: String,
    pub provider: String,
    pub model: String,
    pub temperature: Option<f32>,
    pub skill_ids: Vec<Uuid>,
}
