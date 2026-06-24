use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePermissionCommand {
    pub name: String,
    pub action: String,
    pub resource: String,
    pub description: Option<String>,
}
