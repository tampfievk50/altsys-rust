use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCredentialCommand {
    pub name: Option<String>,
    pub secret: Option<String>,
    pub metadata: Option<String>,
    pub is_active: Option<bool>,
}
