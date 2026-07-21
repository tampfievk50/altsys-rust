use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::CreateWorkflowTemplateCommand::CreateWorkflowTemplateCommand;
use crate::dto::InstantiateTemplateCommand::InstantiateTemplateCommand;
use crate::dto::InstantiateTemplateResponse::InstantiateTemplateResponse;
use crate::dto::UpdateWorkflowTemplateCommand::UpdateWorkflowTemplateCommand;
use crate::dto::WorkflowTemplate::{NewWorkflowTemplate, WorkflowTemplate};
use crate::dto::WorkflowTemplateResponse::WorkflowTemplateResponse;
use crate::port::input::WorkflowTemplatePort::WorkflowTemplatePort;
use crate::port::output::WorkflowTemplateRepositoryPort::WorkflowTemplateRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct WorkflowTemplateService {
    template_repository: Arc<dyn WorkflowTemplateRepositoryPort>,
}

impl WorkflowTemplateService {
    pub fn new(template_repository: Arc<dyn WorkflowTemplateRepositoryPort>) -> Self {
        Self { template_repository }
    }

    fn to_response(template: &WorkflowTemplate) -> WorkflowTemplateResponse {
        WorkflowTemplateResponse {
            id: template.id,
            tenant_id: template.tenant_id,
            key: template.key.clone(),
            version: template.version,
            name: template.name.clone(),
            description: template.description.clone(),
            definition_template: template.definition_template.clone(),
            is_active: template.is_active,
            created_at: template.created_at,
            updated_at: template.updated_at,
            created_by: template.created_by,
            updated_by: template.updated_by,
        }
    }
}

#[async_trait]
impl WorkflowTemplatePort for WorkflowTemplateService {
    async fn create_template(&self, command: CreateWorkflowTemplateCommand) -> Result<WorkflowTemplateResponse, DomainError> {
        info!(key = %command.key, "Creating workflow template version");
        if command.key.trim().is_empty() {
            return Err(DomainError::ValidationError("Key cannot be empty".into()));
        }
        if command.name.trim().is_empty() {
            return Err(DomainError::ValidationError("Name cannot be empty".into()));
        }
        if command.definition_template.trim().is_empty() {
            return Err(DomainError::ValidationError("Definition template cannot be empty".into()));
        }

        let existing_versions = self.template_repository.find_all_by_key_and_tenant(command.tenant_id, &command.key).await?;
        let next_version = existing_versions.iter().map(|t| t.version).max().unwrap_or(0) + 1;

        let template = WorkflowTemplate::new(NewWorkflowTemplate {
            tenant_id: command.tenant_id,
            key: command.key,
            version: next_version,
            name: command.name,
            description: command.description,
            definition_template: command.definition_template,
        });
        self.template_repository.save(&template).await?;
        info!(template_id = %template.id, version = template.version, "Workflow template version created");
        Ok(Self::to_response(&template))
    }

    async fn find_template_by_id(&self, id: Uuid) -> Result<WorkflowTemplateResponse, DomainError> {
        let template = self.template_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Workflow template not found: {}", id)))?;
        Ok(Self::to_response(&template))
    }

    async fn find_latest_template_by_key(&self, tenant_id: Uuid, key: &str) -> Result<WorkflowTemplateResponse, DomainError> {
        let versions = self.template_repository.find_all_by_key_and_tenant(tenant_id, key).await?;
        let latest = versions.into_iter().max_by_key(|t| t.version)
            .ok_or_else(|| DomainError::NotFound(format!("Workflow template not found for key: {}", key)))?;
        Ok(Self::to_response(&latest))
    }

    async fn find_template_versions_by_key(&self, tenant_id: Uuid, key: &str) -> Result<Vec<WorkflowTemplateResponse>, DomainError> {
        let mut versions = self.template_repository.find_all_by_key_and_tenant(tenant_id, key).await?;
        versions.sort_by_key(|t| t.version);
        Ok(versions.iter().map(Self::to_response).collect())
    }

    async fn find_templates_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WorkflowTemplateResponse>, DomainError> {
        let templates = self.template_repository.find_by_tenant(tenant_id).await?;
        Ok(WorkflowTemplate::latest_per_key(templates).iter().map(Self::to_response).collect())
    }

    async fn update_template(&self, id: Uuid, command: UpdateWorkflowTemplateCommand) -> Result<WorkflowTemplateResponse, DomainError> {
        info!(template_id = %id, "Updating workflow template");
        let mut template = self.template_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Workflow template not found: {}", id)))?;
        if let Some(name) = command.name {
            if name.trim().is_empty() {
                return Err(DomainError::ValidationError("Name cannot be empty".into()));
            }
            template.name = name;
        }
        if let Some(description) = command.description {
            template.description = Some(description);
        }
        if let Some(is_active) = command.is_active {
            template.is_active = is_active;
        }
        template.updated_at = Utc::now();
        self.template_repository.update(&template).await?;
        Ok(Self::to_response(&template))
    }

    async fn delete_template(&self, id: Uuid) -> Result<(), DomainError> {
        info!(template_id = %id, "Deleting workflow template");
        let deleted = self.template_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(template_id = %id, "Workflow template not found for deletion");
            return Err(DomainError::NotFound(format!("Workflow template not found: {}", id)));
        }
        Ok(())
    }

    async fn instantiate_template(&self, id: Uuid, command: InstantiateTemplateCommand) -> Result<InstantiateTemplateResponse, DomainError> {
        let template = self.template_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Workflow template not found: {}", id)))?;
        Ok(InstantiateTemplateResponse { definition: template.instantiate(&command.parameters) })
    }
}
