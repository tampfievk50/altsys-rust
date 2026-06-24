use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTenantCommand {
    pub name: Option<String>,
    pub is_active: Option<bool>,
}
