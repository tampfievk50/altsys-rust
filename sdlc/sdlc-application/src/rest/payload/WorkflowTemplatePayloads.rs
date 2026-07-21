use std::collections::HashMap;

use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sdlc_domain::dto::CreateWorkflowTemplateCommand::CreateWorkflowTemplateCommand;
use sdlc_domain::dto::InstantiateTemplateCommand::InstantiateTemplateCommand;
use sdlc_domain::dto::UpdateWorkflowTemplateCommand::UpdateWorkflowTemplateCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateWorkflowTemplateRequest {
    pub tenant_id: Uuid,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub definition_template: String,
}

impl From<CreateWorkflowTemplateRequest> for CreateWorkflowTemplateCommand {
    fn from(val: CreateWorkflowTemplateRequest) -> Self {
        CreateWorkflowTemplateCommand {
            tenant_id: val.tenant_id,
            key: val.key,
            name: val.name,
            description: val.description,
            definition_template: val.definition_template,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateWorkflowTemplateRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

impl From<UpdateWorkflowTemplateRequest> for UpdateWorkflowTemplateCommand {
    fn from(val: UpdateWorkflowTemplateRequest) -> Self {
        UpdateWorkflowTemplateCommand {
            name: val.name,
            description: val.description,
            is_active: val.is_active,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct InstantiateTemplateRequest {
    #[serde(default)]
    pub parameters: HashMap<String, String>,
}

impl From<InstantiateTemplateRequest> for InstantiateTemplateCommand {
    fn from(val: InstantiateTemplateRequest) -> Self {
        InstantiateTemplateCommand { parameters: val.parameters }
    }
}
