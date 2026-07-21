use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::CreateWorkflowDefinitionCommand::CreateWorkflowDefinitionCommand;
use crate::dto::UpdateWorkflowDefinitionCommand::UpdateWorkflowDefinitionCommand;
use crate::dto::WorkflowDefinition::{NewWorkflowDefinition, WorkflowDefinition};
use crate::dto::WorkflowDefinitionResponse::WorkflowDefinitionResponse;
use crate::dto::WorkflowGraph::WorkflowGraph;
use crate::port::input::WorkflowDefinitionPort::WorkflowDefinitionPort;
use crate::port::output::WorkflowDefinitionRepositoryPort::WorkflowDefinitionRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct WorkflowDefinitionService {
    definition_repository: Arc<dyn WorkflowDefinitionRepositoryPort>,
}

impl WorkflowDefinitionService {
    pub fn new(definition_repository: Arc<dyn WorkflowDefinitionRepositoryPort>) -> Self {
        Self { definition_repository }
    }

    fn to_response(definition: &WorkflowDefinition) -> WorkflowDefinitionResponse {
        WorkflowDefinitionResponse {
            id: definition.id,
            tenant_id: definition.tenant_id,
            key: definition.key.clone(),
            version: definition.version,
            name: definition.name.clone(),
            description: definition.description.clone(),
            definition: definition.definition.clone(),
            is_active: definition.is_active,
            created_at: definition.created_at,
            updated_at: definition.updated_at,
            created_by: definition.created_by,
            updated_by: definition.updated_by,
        }
    }
}

#[async_trait]
impl WorkflowDefinitionPort for WorkflowDefinitionService {
    async fn create_workflow_definition(&self, command: CreateWorkflowDefinitionCommand) -> Result<WorkflowDefinitionResponse, DomainError> {
        info!(key = %command.key, "Creating workflow definition version");
        if command.key.trim().is_empty() {
            return Err(DomainError::ValidationError("Key cannot be empty".into()));
        }
        if command.name.trim().is_empty() {
            return Err(DomainError::ValidationError("Name cannot be empty".into()));
        }
        WorkflowGraph::parse(&command.definition)?.validate()?;

        let existing_versions = self.definition_repository.find_all_by_key_and_tenant(command.tenant_id, &command.key).await?;
        let next_version = existing_versions.iter().map(|d| d.version).max().unwrap_or(0) + 1;

        let definition = WorkflowDefinition::new(NewWorkflowDefinition {
            tenant_id: command.tenant_id,
            key: command.key,
            version: next_version,
            name: command.name,
            description: command.description,
            definition: command.definition,
        });
        self.definition_repository.save(&definition).await?;
        info!(definition_id = %definition.id, version = definition.version, "Workflow definition version created");
        Ok(Self::to_response(&definition))
    }

    async fn find_workflow_definition_by_id(&self, id: Uuid) -> Result<WorkflowDefinitionResponse, DomainError> {
        let definition = self.definition_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Workflow definition not found: {}", id)))?;
        Ok(Self::to_response(&definition))
    }

    async fn find_latest_workflow_definition_by_key(&self, tenant_id: Uuid, key: &str) -> Result<WorkflowDefinitionResponse, DomainError> {
        let versions = self.definition_repository.find_all_by_key_and_tenant(tenant_id, key).await?;
        let latest = versions.into_iter().max_by_key(|d| d.version)
            .ok_or_else(|| DomainError::NotFound(format!("Workflow definition not found for key: {}", key)))?;
        Ok(Self::to_response(&latest))
    }

    async fn find_workflow_definition_versions_by_key(&self, tenant_id: Uuid, key: &str) -> Result<Vec<WorkflowDefinitionResponse>, DomainError> {
        let mut versions = self.definition_repository.find_all_by_key_and_tenant(tenant_id, key).await?;
        versions.sort_by_key(|d| d.version);
        Ok(versions.iter().map(Self::to_response).collect())
    }

    async fn find_workflow_definitions_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WorkflowDefinitionResponse>, DomainError> {
        let definitions = self.definition_repository.find_by_tenant(tenant_id).await?;
        Ok(WorkflowDefinition::latest_per_key(definitions).iter().map(Self::to_response).collect())
    }

    async fn update_workflow_definition(&self, id: Uuid, command: UpdateWorkflowDefinitionCommand) -> Result<WorkflowDefinitionResponse, DomainError> {
        info!(definition_id = %id, "Updating workflow definition");
        let mut definition = self.definition_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Workflow definition not found: {}", id)))?;
        if let Some(name) = command.name {
            if name.trim().is_empty() {
                return Err(DomainError::ValidationError("Name cannot be empty".into()));
            }
            definition.name = name;
        }
        if let Some(description) = command.description {
            definition.description = Some(description);
        }
        if let Some(is_active) = command.is_active {
            definition.is_active = is_active;
        }
        definition.updated_at = Utc::now();
        self.definition_repository.update(&definition).await?;
        Ok(Self::to_response(&definition))
    }

    async fn delete_workflow_definition(&self, id: Uuid) -> Result<(), DomainError> {
        info!(definition_id = %id, "Deleting workflow definition");
        let deleted = self.definition_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(definition_id = %id, "Workflow definition not found for deletion");
            return Err(DomainError::NotFound(format!("Workflow definition not found: {}", id)));
        }
        Ok(())
    }
}
