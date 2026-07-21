use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::CreateWorkflowDefinitionCommand::CreateWorkflowDefinitionCommand;
use crate::dto::UpdateWorkflowDefinitionCommand::UpdateWorkflowDefinitionCommand;
use crate::dto::WorkflowDefinitionResponse::WorkflowDefinitionResponse;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait WorkflowDefinitionPort: Send + Sync {
    /// Validates `command.definition` as a `WorkflowGraph` and creates a new version
    /// for `command.key` (max existing version for the key, tenant + 1).
    async fn create_workflow_definition(&self, command: CreateWorkflowDefinitionCommand) -> Result<WorkflowDefinitionResponse, DomainError>;
    async fn find_workflow_definition_by_id(&self, id: Uuid) -> Result<WorkflowDefinitionResponse, DomainError>;
    async fn find_latest_workflow_definition_by_key(&self, tenant_id: Uuid, key: &str) -> Result<WorkflowDefinitionResponse, DomainError>;
    async fn find_workflow_definition_versions_by_key(&self, tenant_id: Uuid, key: &str) -> Result<Vec<WorkflowDefinitionResponse>, DomainError>;
    async fn find_workflow_definitions_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WorkflowDefinitionResponse>, DomainError>;
    async fn update_workflow_definition(&self, id: Uuid, command: UpdateWorkflowDefinitionCommand) -> Result<WorkflowDefinitionResponse, DomainError>;
    async fn delete_workflow_definition(&self, id: Uuid) -> Result<(), DomainError>;
}
