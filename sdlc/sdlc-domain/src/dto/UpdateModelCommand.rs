use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateModelCommand {
    pub provider: Option<String>,
    pub model_name: Option<String>,
    pub capability: Option<String>,
    pub credential_id: Option<Uuid>,
    pub endpoint_url: Option<String>,
    pub is_active: Option<bool>,
}
