use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::dto::AgentType::AgentType;

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: Uuid,
    /// `None` marks a global/platform-wide agent available to all tenants.
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub agent_type: AgentType,
    pub system_prompt: String,
    /// Selects the `LlmClientPort` implementation to dispatch to (e.g. `"openai"`, `"anthropic"`, `"echo"`).
    pub provider: String,
    pub model: String,
    pub temperature: Option<f32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

pub struct NewAgent {
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub agent_type: AgentType,
    pub system_prompt: String,
    pub provider: String,
    pub model: String,
    pub temperature: Option<f32>,
}

impl Agent {
    pub fn new(fields: NewAgent) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: fields.tenant_id,
            name: fields.name,
            agent_type: fields.agent_type,
            system_prompt: fields.system_prompt,
            provider: fields.provider,
            model: fields.model,
            temperature: fields.temperature,
            is_active: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}
