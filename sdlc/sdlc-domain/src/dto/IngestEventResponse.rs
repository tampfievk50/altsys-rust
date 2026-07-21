use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::dto::EventResponse::EventResponse;
use crate::dto::RuleFiringResponse::RuleFiringResponse;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct IngestEventResponse {
    pub event: EventResponse,
    pub firings: Vec<RuleFiringResponse>,
}
