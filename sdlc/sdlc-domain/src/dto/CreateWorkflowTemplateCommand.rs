use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkflowTemplateCommand {
    pub tenant_id: Uuid,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub definition_template: String,
}
