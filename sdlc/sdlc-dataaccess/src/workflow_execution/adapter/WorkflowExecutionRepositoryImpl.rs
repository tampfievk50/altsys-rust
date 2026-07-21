use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::WorkflowExecution::WorkflowExecution;
use sdlc_domain::port::output::WorkflowExecutionRepositoryPort::WorkflowExecutionRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::workflow_execution::mapper::WorkflowExecutionDataMapper::WorkflowExecutionDataMapper;
use crate::workflow_execution::repository::WorkflowExecutionSeaOrmRepository::WorkflowExecutionSeaOrmRepository;

pub struct WorkflowExecutionRepositoryImpl {
    sea_orm_repo: WorkflowExecutionSeaOrmRepository,
}

impl WorkflowExecutionRepositoryImpl {
    pub fn new(sea_orm_repo: WorkflowExecutionSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl WorkflowExecutionRepositoryPort for WorkflowExecutionRepositoryImpl {
    async fn save(&self, execution: &WorkflowExecution) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(WorkflowExecutionDataMapper::to_active_model(execution)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save workflow execution"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, execution: &WorkflowExecution) -> Result<(), DomainError> {
        self.sea_orm_repo.update(WorkflowExecutionDataMapper::to_active_model(execution)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update workflow execution"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<WorkflowExecution>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(WorkflowExecutionDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find workflow execution"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WorkflowExecution>, DomainError> {
        self.sea_orm_repo.find_by_tenant(tenant_id).await
            .map(|models| models.iter().map(WorkflowExecutionDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list workflow executions"); DomainError::InternalError(e.to_string()) })
    }
}
