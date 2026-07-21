use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sdlc_domain::dto::CreateWorkflowDefinitionCommand::CreateWorkflowDefinitionCommand;
use sdlc_domain::dto::UpdateWorkflowDefinitionCommand::UpdateWorkflowDefinitionCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateWorkflowDefinitionRequest {
    pub tenant_id: Uuid,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub definition: String,
}

impl From<CreateWorkflowDefinitionRequest> for CreateWorkflowDefinitionCommand {
    fn from(val: CreateWorkflowDefinitionRequest) -> Self {
        CreateWorkflowDefinitionCommand {
            tenant_id: val.tenant_id,
            key: val.key,
            name: val.name,
            description: val.description,
            definition: val.definition,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateWorkflowDefinitionRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

impl From<UpdateWorkflowDefinitionRequest> for UpdateWorkflowDefinitionCommand {
    fn from(val: UpdateWorkflowDefinitionRequest) -> Self {
        UpdateWorkflowDefinitionCommand {
            name: val.name,
            description: val.description,
            is_active: val.is_active,
        }
    }
}
