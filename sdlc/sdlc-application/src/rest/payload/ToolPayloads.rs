use std::collections::HashMap;

use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sdlc_domain::dto::CreateToolCommand::CreateToolCommand;
use sdlc_domain::dto::ExecuteToolCommand::ExecuteToolCommand;
use sdlc_domain::dto::UpdateToolCommand::UpdateToolCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateToolRequest {
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub tool_type: String,
    pub description: Option<String>,
    pub config: Option<String>,
}

impl From<CreateToolRequest> for CreateToolCommand {
    fn from(val: CreateToolRequest) -> Self {
        CreateToolCommand {
            tenant_id: val.tenant_id,
            name: val.name,
            tool_type: val.tool_type,
            description: val.description,
            config: val.config,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateToolRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub config: Option<String>,
    pub is_enabled: Option<bool>,
}

impl From<UpdateToolRequest> for UpdateToolCommand {
    fn from(val: UpdateToolRequest) -> Self {
        UpdateToolCommand {
            name: val.name,
            description: val.description,
            config: val.config,
            is_enabled: val.is_enabled,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExecuteToolRequest {
    pub action: String,
    #[serde(default)]
    pub parameters: HashMap<String, String>,
    pub working_directory: Option<String>,
}

impl From<ExecuteToolRequest> for ExecuteToolCommand {
    fn from(val: ExecuteToolRequest) -> Self {
        ExecuteToolCommand {
            action: val.action,
            parameters: val.parameters,
            working_directory: val.working_directory,
        }
    }
}
