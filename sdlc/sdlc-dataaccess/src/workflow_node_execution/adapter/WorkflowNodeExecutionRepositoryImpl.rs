use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::WorkflowNodeExecution::WorkflowNodeExecution;
use sdlc_domain::port::output::WorkflowNodeExecutionRepositoryPort::WorkflowNodeExecutionRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::workflow_node_execution::mapper::WorkflowNodeExecutionDataMapper::WorkflowNodeExecutionDataMapper;
use crate::workflow_node_execution::repository::WorkflowNodeExecutionSeaOrmRepository::WorkflowNodeExecutionSeaOrmRepository;

pub struct WorkflowNodeExecutionRepositoryImpl {
    sea_orm_repo: WorkflowNodeExecutionSeaOrmRepository,
}

impl WorkflowNodeExecutionRepositoryImpl {
    pub fn new(sea_orm_repo: WorkflowNodeExecutionSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl WorkflowNodeExecutionRepositoryPort for WorkflowNodeExecutionRepositoryImpl {
    async fn save(&self, node_execution: &WorkflowNodeExecution) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(WorkflowNodeExecutionDataMapper::to_active_model(node_execution)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save workflow node execution"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, node_execution: &WorkflowNodeExecution) -> Result<(), DomainError> {
        self.sea_orm_repo.update(WorkflowNodeExecutionDataMapper::to_active_model(node_execution)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update workflow node execution"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_execution_id(&self, execution_id: Uuid) -> Result<Vec<WorkflowNodeExecution>, DomainError> {
        self.sea_orm_repo.find_by_execution_id(execution_id).await
            .map(|models| models.iter().map(WorkflowNodeExecutionDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list workflow node executions"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_latest_by_execution_and_node(&self, execution_id: Uuid, node_id: &str) -> Result<Option<WorkflowNodeExecution>, DomainError> {
        self.sea_orm_repo.find_latest_by_execution_and_node(execution_id, node_id).await
            .map(|opt| opt.as_ref().map(WorkflowNodeExecutionDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find workflow node execution"); DomainError::InternalError(e.to_string()) })
    }
}
