use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionCommand {
    pub name: Option<String>,
    pub description: Option<String>,
}
