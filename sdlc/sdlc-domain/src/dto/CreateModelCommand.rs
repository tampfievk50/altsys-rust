use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateModelCommand {
    pub tenant_id: Option<Uuid>,
    pub provider: String,
    pub model_name: String,
    pub capability: String,
    pub credential_id: Option<Uuid>,
    pub endpoint_url: Option<String>,
}
