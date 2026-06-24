use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserCommand {
    pub tenant_id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
}
