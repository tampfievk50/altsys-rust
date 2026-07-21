use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateToolCommand {
    pub name: Option<String>,
    pub description: Option<String>,
    pub config: Option<String>,
    pub is_enabled: Option<bool>,
}
