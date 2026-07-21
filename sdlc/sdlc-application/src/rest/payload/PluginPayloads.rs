use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sdlc_domain::dto::CreatePluginCommand::CreatePluginCommand;
use sdlc_domain::dto::UpdatePluginCommand::UpdatePluginCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePluginRequest {
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub webhook_url: String,
    pub secret: Option<String>,
}

impl From<CreatePluginRequest> for CreatePluginCommand {
    fn from(val: CreatePluginRequest) -> Self {
        CreatePluginCommand {
            tenant_id: val.tenant_id,
            name: val.name,
            webhook_url: val.webhook_url,
            secret: val.secret,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePluginRequest {
    pub name: Option<String>,
    pub webhook_url: Option<String>,
    pub secret: Option<String>,
    pub is_active: Option<bool>,
}

impl From<UpdatePluginRequest> for UpdatePluginCommand {
    fn from(val: UpdatePluginRequest) -> Self {
        UpdatePluginCommand {
            name: val.name,
            webhook_url: val.webhook_url,
            secret: val.secret,
            is_active: val.is_active,
        }
    }
}
