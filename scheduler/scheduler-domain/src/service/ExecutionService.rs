use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::ExecutionResponse::ExecutionResponse;
use crate::dto::JobExecution::JobExecution;
use crate::port::input::ExecutionPort::ExecutionPort;
use crate::port::output::ExecutionRepositoryPort::ExecutionRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct ExecutionService {
    execution_repository: Arc<dyn ExecutionRepositoryPort>,
}

impl ExecutionService {
    pub fn new(execution_repository: Arc<dyn ExecutionRepositoryPort>) -> Self {
        Self { execution_repository }
    }

    fn to_response(execution: &JobExecution) -> ExecutionResponse {
        ExecutionResponse {
            id: execution.id,
            scheduler_id: execution.scheduler_id,
            trigger_type: execution.trigger_type.clone(),
            status: execution.status.clone(),
            started_at: execution.started_at,
            finished_at: execution.finished_at,
            status_code: execution.status_code,
            response_body: execution.response_body.clone(),
            error_message: execution.error_message.clone(),
        }
    }
}

#[async_trait]
impl ExecutionPort for ExecutionService {
    async fn find_executions_by_scheduler(&self, scheduler_id: Uuid) -> Result<Vec<ExecutionResponse>, DomainError> {
        let executions = self.execution_repository.find_by_scheduler_id(scheduler_id).await?;
        Ok(executions.iter().map(Self::to_response).collect())
    }

    async fn find_execution_by_id(&self, id: Uuid) -> Result<ExecutionResponse, DomainError> {
        let execution = self
            .execution_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Execution not found: {}", id)))?;
        Ok(Self::to_response(&execution))
    }
}
