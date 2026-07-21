use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::CreateWorkflowTemplateCommand::CreateWorkflowTemplateCommand;
use crate::dto::InstantiateTemplateCommand::InstantiateTemplateCommand;
use crate::dto::InstantiateTemplateResponse::InstantiateTemplateResponse;
use crate::dto::UpdateWorkflowTemplateCommand::UpdateWorkflowTemplateCommand;
use crate::dto::WorkflowTemplateResponse::WorkflowTemplateResponse;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait WorkflowTemplatePort: Send + Sync {
    /// Creates a new template version for `command.key` (max existing version for
    /// the key, tenant + 1).
    async fn create_template(&self, command: CreateWorkflowTemplateCommand) -> Result<WorkflowTemplateResponse, DomainError>;
    async fn find_template_by_id(&self, id: Uuid) -> Result<WorkflowTemplateResponse, DomainError>;
    async fn find_latest_template_by_key(&self, tenant_id: Uuid, key: &str) -> Result<WorkflowTemplateResponse, DomainError>;
    async fn find_template_versions_by_key(&self, tenant_id: Uuid, key: &str) -> Result<Vec<WorkflowTemplateResponse>, DomainError>;
    async fn find_templates_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WorkflowTemplateResponse>, DomainError>;
    async fn update_template(&self, id: Uuid, command: UpdateWorkflowTemplateCommand) -> Result<WorkflowTemplateResponse, DomainError>;
    async fn delete_template(&self, id: Uuid) -> Result<(), DomainError>;
    /// Resolves `{{parameter}}` placeholders; does not call the Workflow service.
    async fn instantiate_template(&self, id: Uuid, command: InstantiateTemplateCommand) -> Result<InstantiateTemplateResponse, DomainError>;
}
