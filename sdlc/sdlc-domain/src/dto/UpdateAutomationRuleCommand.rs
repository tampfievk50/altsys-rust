use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAutomationRuleCommand {
    pub name: Option<String>,
    pub match_criteria: Option<String>,
    pub action: Option<String>,
    pub is_active: Option<bool>,
}
