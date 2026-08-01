use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use scheduler_domain::dto::JobExecution::JobExecution;
use scheduler_domain::port::output::ExecutionRepositoryPort::ExecutionRepositoryPort;
use scheduler_domain::r#enum::DomainError::DomainError;

use crate::job_execution::mapper::JobExecutionDataMapper::JobExecutionDataMapper;
use crate::job_execution::repository::JobExecutionSeaOrmRepository::JobExecutionSeaOrmRepository;

pub struct JobExecutionRepositoryImpl {
    sea_orm_repo: JobExecutionSeaOrmRepository,
}

impl JobExecutionRepositoryImpl {
    pub fn new(sea_orm_repo: JobExecutionSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl ExecutionRepositoryPort for JobExecutionRepositoryImpl {
    async fn save(&self, execution: &JobExecution) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(JobExecutionDataMapper::to_active_model(execution)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save execution"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, execution: &JobExecution) -> Result<(), DomainError> {
        self.sea_orm_repo.update(JobExecutionDataMapper::to_active_model(execution)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update execution"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<JobExecution>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(JobExecutionDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find execution"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_scheduler_id(&self, scheduler_id: Uuid) -> Result<Vec<JobExecution>, DomainError> {
        self.sea_orm_repo.find_by_scheduler_id(scheduler_id).await
            .map(|models| models.iter().map(JobExecutionDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list executions"); DomainError::InternalError(e.to_string()) })
    }
}
