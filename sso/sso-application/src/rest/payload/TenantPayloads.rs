use serde::Deserialize;
use utoipa::ToSchema;

use sso_domain::dto::CreateTenantCommand::CreateTenantCommand;
use sso_domain::dto::UpdateTenantCommand::UpdateTenantCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTenantRequest {
    pub name: String,
    pub slug: String,
}

impl Into<CreateTenantCommand> for CreateTenantRequest {
    fn into(self) -> CreateTenantCommand {
        CreateTenantCommand {
            name: self.name,
            slug: self.slug,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTenantRequest {
    pub name: Option<String>,
    pub is_active: Option<bool>,
}

impl Into<UpdateTenantCommand> for UpdateTenantRequest {
    fn into(self) -> UpdateTenantCommand {
        UpdateTenantCommand {
            name: self.name,
            is_active: self.is_active,
        }
    }
}
