use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSkillCommand {
    pub name: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub is_active: Option<bool>,
}
