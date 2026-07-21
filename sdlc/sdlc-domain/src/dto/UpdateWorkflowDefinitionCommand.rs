use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkflowDefinitionCommand {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}
