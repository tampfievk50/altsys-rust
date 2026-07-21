use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sdlc_domain::dto::IngestEventCommand::IngestEventCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct IngestEventRequest {
    pub tenant_id: Uuid,
    pub event_type: String,
    #[serde(default)]
    pub payload: serde_json::Value,
}

impl From<IngestEventRequest> for IngestEventCommand {
    fn from(val: IngestEventRequest) -> Self {
        IngestEventCommand {
            tenant_id: val.tenant_id,
            event_type: val.event_type,
            payload: val.payload,
        }
    }
}
