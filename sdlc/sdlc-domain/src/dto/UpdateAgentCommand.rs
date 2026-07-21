use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentCommand {
    pub name: Option<String>,
    pub system_prompt: Option<String>,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub is_active: Option<bool>,
    /// `None` leaves attached skills unchanged; `Some(ids)` fully replaces them (an empty vec clears all).
    pub skill_ids: Option<Vec<Uuid>>,
}
