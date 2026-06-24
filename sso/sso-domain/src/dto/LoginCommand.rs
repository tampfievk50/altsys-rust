use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginCommand {
    pub tenant_slug: String,
    pub username: String,
    pub password: String,
}
