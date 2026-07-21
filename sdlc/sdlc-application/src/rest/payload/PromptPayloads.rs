use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sdlc_domain::dto::CreatePromptCommand::CreatePromptCommand;
use sdlc_domain::dto::UpdatePromptCommand::UpdatePromptCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePromptRequest {
    pub tenant_id: Uuid,
    pub key: String,
    pub content: String,
    pub variables: Option<String>,
    pub description: Option<String>,
}

impl From<CreatePromptRequest> for CreatePromptCommand {
    fn from(val: CreatePromptRequest) -> Self {
        CreatePromptCommand {
            tenant_id: val.tenant_id,
            key: val.key,
            content: val.content,
            variables: val.variables,
            description: val.description,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePromptRequest {
    pub content: Option<String>,
    pub variables: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

impl From<UpdatePromptRequest> for UpdatePromptCommand {
    fn from(val: UpdatePromptRequest) -> Self {
        UpdatePromptCommand {
            content: val.content,
            variables: val.variables,
            description: val.description,
            is_active: val.is_active,
        }
    }
}
