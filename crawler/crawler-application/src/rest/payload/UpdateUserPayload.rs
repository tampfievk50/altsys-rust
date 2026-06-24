use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserPayload {
    pub email: Option<String>,
    pub password: Option<String>,
    pub full_name: Option<String>,
    pub is_active: Option<bool>,
}
