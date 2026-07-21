use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::WorkflowTemplate::WorkflowTemplate;
use sdlc_domain::port::output::WorkflowTemplateRepositoryPort::WorkflowTemplateRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::workflow_template::mapper::WorkflowTemplateDataMapper::WorkflowTemplateDataMapper;
use crate::workflow_template::repository::WorkflowTemplateSeaOrmRepository::WorkflowTemplateSeaOrmRepository;

pub struct WorkflowTemplateRepositoryImpl {
    sea_orm_repo: WorkflowTemplateSeaOrmRepository,
}

impl WorkflowTemplateRepositoryImpl {
    pub fn new(sea_orm_repo: WorkflowTemplateSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl WorkflowTemplateRepositoryPort for WorkflowTemplateRepositoryImpl {
    async fn save(&self, template: &WorkflowTemplate) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(WorkflowTemplateDataMapper::to_active_model(template)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save workflow template"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, template: &WorkflowTemplate) -> Result<(), DomainError> {
        self.sea_orm_repo.update(WorkflowTemplateDataMapper::to_active_model(template)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update workflow template"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<WorkflowTemplate>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(WorkflowTemplateDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find workflow template"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_all_by_key_and_tenant(&self, tenant_id: Uuid, key: &str) -> Result<Vec<WorkflowTemplate>, DomainError> {
        self.sea_orm_repo.find_all_by_key_and_tenant(tenant_id, key).await
            .map(|models| models.iter().map(WorkflowTemplateDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list workflow template versions"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WorkflowTemplate>, DomainError> {
        self.sea_orm_repo.find_by_tenant(tenant_id).await
            .map(|models| models.iter().map(WorkflowTemplateDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list workflow templates"); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(error = %e, "Failed to delete workflow template"); DomainError::InternalError(e.to_string()) })
    }
}
