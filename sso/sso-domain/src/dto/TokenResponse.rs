use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}
