use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePromptCommand {
    pub content: Option<String>,
    pub variables: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}
