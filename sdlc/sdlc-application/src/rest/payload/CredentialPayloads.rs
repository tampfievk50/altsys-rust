use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sdlc_domain::dto::CreateCredentialCommand::CreateCredentialCommand;
use sdlc_domain::dto::UpdateCredentialCommand::UpdateCredentialCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCredentialRequest {
    pub tenant_id: Uuid,
    pub name: String,
    pub provider: String,
    pub secret: String,
    pub metadata: Option<String>,
}

impl From<CreateCredentialRequest> for CreateCredentialCommand {
    fn from(val: CreateCredentialRequest) -> Self {
        CreateCredentialCommand {
            tenant_id: val.tenant_id,
            name: val.name,
            provider: val.provider,
            secret: val.secret,
            metadata: val.metadata,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateCredentialRequest {
    pub name: Option<String>,
    pub secret: Option<String>,
    pub metadata: Option<String>,
    pub is_active: Option<bool>,
}

impl From<UpdateCredentialRequest> for UpdateCredentialCommand {
    fn from(val: UpdateCredentialRequest) -> Self {
        UpdateCredentialCommand {
            name: val.name,
            secret: val.secret,
            metadata: val.metadata,
            is_active: val.is_active,
        }
    }
}
