use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sdlc_domain::dto::ExecuteAgentCommand::ExecuteAgentCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExecuteAgentRequest {
    pub tenant_id: Uuid,
    pub input: String,
}

impl From<ExecuteAgentRequest> for ExecuteAgentCommand {
    fn from(val: ExecuteAgentRequest) -> Self {
        ExecuteAgentCommand {
            tenant_id: val.tenant_id,
            input: val.input,
        }
    }
}
