use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserCommand {
    pub email: Option<String>,
    pub password: Option<String>,
    pub full_name: Option<String>,
    pub is_active: Option<bool>,
}
