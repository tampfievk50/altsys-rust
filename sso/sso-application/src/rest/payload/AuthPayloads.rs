use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use sso_domain::dto::LoginCommand::LoginCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub tenant_slug: String,
    pub username: String,
    pub password: String,
}

impl Into<LoginCommand> for LoginRequest {
    fn into(self) -> LoginCommand {
        LoginCommand {
            tenant_slug: self.tenant_slug,
            username: self.username,
            password: self.password,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LogoutRequest {
    pub refresh_token: String,
}
