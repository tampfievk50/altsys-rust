use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateUserPayload {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
}
