use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sso_domain::dto::CreateRoleCommand::CreateRoleCommand;
use sso_domain::dto::UpdateRoleCommand::UpdateRoleCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRoleRequest {
    pub tenant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

impl Into<CreateRoleCommand> for CreateRoleRequest {
    fn into(self) -> CreateRoleCommand {
        CreateRoleCommand {
            tenant_id: self.tenant_id,
            name: self.name,
            description: self.description,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateRoleRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Into<UpdateRoleCommand> for UpdateRoleRequest {
    fn into(self) -> UpdateRoleCommand {
        UpdateRoleCommand {
            name: self.name,
            description: self.description,
        }
    }
}
