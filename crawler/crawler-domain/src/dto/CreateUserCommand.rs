use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserCommand {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
}
