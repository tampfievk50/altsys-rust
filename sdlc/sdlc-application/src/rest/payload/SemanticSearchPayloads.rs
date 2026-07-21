use serde::Deserialize;
use utoipa::ToSchema;

use sdlc_domain::dto::SemanticSearchCommand::SemanticSearchCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct SemanticSearchRequest {
    pub query: String,
    pub source_type: Option<String>,
    pub limit: Option<u32>,
}

impl From<SemanticSearchRequest> for SemanticSearchCommand {
    fn from(val: SemanticSearchRequest) -> Self {
        SemanticSearchCommand {
            query: val.query,
            source_type: val.source_type,
            limit: val.limit,
        }
    }
}
