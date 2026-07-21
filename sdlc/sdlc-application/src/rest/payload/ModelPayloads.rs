use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sdlc_domain::dto::CreateModelCommand::CreateModelCommand;
use sdlc_domain::dto::UpdateModelCommand::UpdateModelCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateModelRequest {
    pub tenant_id: Option<Uuid>,
    pub provider: String,
    pub model_name: String,
    pub capability: String,
    pub credential_id: Option<Uuid>,
    pub endpoint_url: Option<String>,
}

impl From<CreateModelRequest> for CreateModelCommand {
    fn from(val: CreateModelRequest) -> Self {
        CreateModelCommand {
            tenant_id: val.tenant_id,
            provider: val.provider,
            model_name: val.model_name,
            capability: val.capability,
            credential_id: val.credential_id,
            endpoint_url: val.endpoint_url,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateModelRequest {
    pub provider: Option<String>,
    pub model_name: Option<String>,
    pub capability: Option<String>,
    pub credential_id: Option<Uuid>,
    pub endpoint_url: Option<String>,
    pub is_active: Option<bool>,
}

impl From<UpdateModelRequest> for UpdateModelCommand {
    fn from(val: UpdateModelRequest) -> Self {
        UpdateModelCommand {
            provider: val.provider,
            model_name: val.model_name,
            capability: val.capability,
            credential_id: val.credential_id,
            endpoint_url: val.endpoint_url,
            is_active: val.is_active,
        }
    }
}
