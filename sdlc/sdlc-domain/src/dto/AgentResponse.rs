use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::dto::AgentType::AgentType;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AgentResponse {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub agent_type: AgentType,
    pub system_prompt: String,
    pub provider: String,
    pub model: String,
    pub temperature: Option<f32>,
    pub is_active: bool,
    pub skill_ids: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}
