use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestEventCommand {
    pub tenant_id: Uuid,
    pub event_type: String,
    #[serde(default)]
    pub payload: serde_json::Value,
}
