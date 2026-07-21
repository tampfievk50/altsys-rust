use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::TaskOverride::TaskOverride;
use sdlc_domain::port::output::TaskOverrideRepositoryPort::TaskOverrideRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::task_override::mapper::TaskOverrideDataMapper::TaskOverrideDataMapper;
use crate::task_override::repository::TaskOverrideSeaOrmRepository::TaskOverrideSeaOrmRepository;

pub struct TaskOverrideRepositoryImpl {
    sea_orm_repo: TaskOverrideSeaOrmRepository,
}

impl TaskOverrideRepositoryImpl {
    pub fn new(sea_orm_repo: TaskOverrideSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl TaskOverrideRepositoryPort for TaskOverrideRepositoryImpl {
    async fn find_by_project(&self, project_id: Uuid) -> Result<Vec<TaskOverride>, DomainError> {
        self.sea_orm_repo.find_by_project(project_id).await
            .map(|models| models.iter().map(TaskOverrideDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list task overrides"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_project_and_ticket(&self, project_id: Uuid, ticket_key: &str) -> Result<Option<TaskOverride>, DomainError> {
        self.sea_orm_repo.find_by_project_and_ticket(project_id, ticket_key).await
            .map(|opt| opt.as_ref().map(TaskOverrideDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find task override"); DomainError::InternalError(e.to_string()) })
    }

    async fn save(&self, task_override: &TaskOverride) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(TaskOverrideDataMapper::to_active_model(task_override)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save task override"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, task_override: &TaskOverride) -> Result<(), DomainError> {
        self.sea_orm_repo.update(TaskOverrideDataMapper::to_active_model(task_override)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update task override"); DomainError::InternalError(e.to_string()) })
    }
}
