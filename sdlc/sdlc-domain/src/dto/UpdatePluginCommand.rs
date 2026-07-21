use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePluginCommand {
    pub name: Option<String>,
    pub webhook_url: Option<String>,
    pub secret: Option<String>,
    pub is_active: Option<bool>,
}
