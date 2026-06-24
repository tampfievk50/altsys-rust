use serde::Deserialize;
use utoipa::ToSchema;

use sso_domain::dto::CreatePermissionCommand::CreatePermissionCommand;
use sso_domain::dto::UpdatePermissionCommand::UpdatePermissionCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePermissionRequest {
    pub name: String,
    pub action: String,
    pub resource: String,
    pub description: Option<String>,
}

impl Into<CreatePermissionCommand> for CreatePermissionRequest {
    fn into(self) -> CreatePermissionCommand {
        CreatePermissionCommand {
            name: self.name,
            action: self.action,
            resource: self.resource,
            description: self.description,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePermissionRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Into<UpdatePermissionCommand> for UpdatePermissionRequest {
    fn into(self) -> UpdatePermissionCommand {
        UpdatePermissionCommand {
            name: self.name,
            description: self.description,
        }
    }
}
