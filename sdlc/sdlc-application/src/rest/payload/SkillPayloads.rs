use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sdlc_domain::dto::CreateSkillCommand::CreateSkillCommand;
use sdlc_domain::dto::UpdateSkillCommand::UpdateSkillCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSkillRequest {
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub content: String,
}

impl From<CreateSkillRequest> for CreateSkillCommand {
    fn from(val: CreateSkillRequest) -> Self {
        CreateSkillCommand {
            tenant_id: val.tenant_id,
            name: val.name,
            description: val.description,
            content: val.content,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSkillRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub is_active: Option<bool>,
}

impl From<UpdateSkillRequest> for UpdateSkillCommand {
    fn from(val: UpdateSkillRequest) -> Self {
        UpdateSkillCommand {
            name: val.name,
            description: val.description,
            content: val.content,
            is_active: val.is_active,
        }
    }
}
