use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::WorkflowDefinition::WorkflowDefinition;
use sdlc_domain::port::output::WorkflowDefinitionRepositoryPort::WorkflowDefinitionRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::workflow_definition::mapper::WorkflowDefinitionDataMapper::WorkflowDefinitionDataMapper;
use crate::workflow_definition::repository::WorkflowDefinitionSeaOrmRepository::WorkflowDefinitionSeaOrmRepository;

pub struct WorkflowDefinitionRepositoryImpl {
    sea_orm_repo: WorkflowDefinitionSeaOrmRepository,
}

impl WorkflowDefinitionRepositoryImpl {
    pub fn new(sea_orm_repo: WorkflowDefinitionSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl WorkflowDefinitionRepositoryPort for WorkflowDefinitionRepositoryImpl {
    async fn save(&self, definition: &WorkflowDefinition) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(WorkflowDefinitionDataMapper::to_active_model(definition)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save workflow definition"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, definition: &WorkflowDefinition) -> Result<(), DomainError> {
        self.sea_orm_repo.update(WorkflowDefinitionDataMapper::to_active_model(definition)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update workflow definition"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<WorkflowDefinition>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(WorkflowDefinitionDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find workflow definition"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_all_by_key_and_tenant(&self, tenant_id: Uuid, key: &str) -> Result<Vec<WorkflowDefinition>, DomainError> {
        self.sea_orm_repo.find_all_by_key_and_tenant(tenant_id, key).await
            .map(|models| models.iter().map(WorkflowDefinitionDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list workflow definition versions"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WorkflowDefinition>, DomainError> {
        self.sea_orm_repo.find_by_tenant(tenant_id).await
            .map(|models| models.iter().map(WorkflowDefinitionDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list workflow definitions"); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(error = %e, "Failed to delete workflow definition"); DomainError::InternalError(e.to_string()) })
    }
}
