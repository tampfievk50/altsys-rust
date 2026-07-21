use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::info;
use uuid::Uuid;

use crate::dto::TaskOverride::{NewTaskOverride, TaskOverride};
use crate::dto::TaskOverrideResponse::TaskOverrideResponse;
use crate::port::input::TaskOverridePort::TaskOverridePort;
use crate::port::output::TaskOverrideRepositoryPort::TaskOverrideRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct TaskOverrideService {
    task_override_repository: Arc<dyn TaskOverrideRepositoryPort>,
}

impl TaskOverrideService {
    pub fn new(task_override_repository: Arc<dyn TaskOverrideRepositoryPort>) -> Self {
        Self { task_override_repository }
    }

    fn to_response(task_override: &TaskOverride) -> TaskOverrideResponse {
        TaskOverrideResponse {
            id: task_override.id,
            project_id: task_override.project_id,
            ticket_key: task_override.ticket_key.clone(),
            summary: task_override.summary.clone(),
            created_at: task_override.created_at,
            updated_at: task_override.updated_at,
        }
    }
}

#[async_trait]
impl TaskOverridePort for TaskOverrideService {
    async fn find_overrides_by_project(&self, project_id: Uuid) -> Result<Vec<TaskOverrideResponse>, DomainError> {
        let overrides = self.task_override_repository.find_by_project(project_id).await?;
        Ok(overrides.iter().map(Self::to_response).collect())
    }

    async fn set_summary_override(&self, project_id: Uuid, ticket_key: String, summary: String) -> Result<TaskOverrideResponse, DomainError> {
        if summary.trim().is_empty() {
            return Err(DomainError::ValidationError("Summary cannot be empty".into()));
        }
        match self.task_override_repository.find_by_project_and_ticket(project_id, &ticket_key).await? {
            Some(mut existing) => {
                existing.summary = summary;
                existing.updated_at = Utc::now();
                self.task_override_repository.update(&existing).await?;
                info!(project_id = %project_id, ticket_key = %existing.ticket_key, "Task summary override updated");
                Ok(Self::to_response(&existing))
            }
            None => {
                let created = TaskOverride::new(NewTaskOverride { project_id, ticket_key, summary });
                self.task_override_repository.save(&created).await?;
                info!(project_id = %project_id, ticket_key = %created.ticket_key, "Task summary override created");
                Ok(Self::to_response(&created))
            }
        }
    }
}
