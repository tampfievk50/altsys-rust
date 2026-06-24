use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sso_domain::dto::CreateUserCommand::CreateUserCommand;
use sso_domain::dto::UpdateUserCommand::UpdateUserCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub tenant_id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
}

impl Into<CreateUserCommand> for CreateUserRequest {
    fn into(self) -> CreateUserCommand {
        CreateUserCommand {
            tenant_id: self.tenant_id,
            username: self.username,
            email: self.email,
            password: self.password,
            full_name: self.full_name,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub password: Option<String>,
    pub full_name: Option<String>,
    pub is_active: Option<bool>,
}

impl Into<UpdateUserCommand> for UpdateUserRequest {
    fn into(self) -> UpdateUserCommand {
        UpdateUserCommand {
            email: self.email,
            password: self.password,
            full_name: self.full_name,
            is_active: self.is_active,
        }
    }
}
