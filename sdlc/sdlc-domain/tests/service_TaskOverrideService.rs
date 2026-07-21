use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use uuid::Uuid;

use sdlc_domain::dto::TaskOverride::TaskOverride;
use sdlc_domain::port::input::TaskOverridePort::TaskOverridePort;
use sdlc_domain::port::output::TaskOverrideRepositoryPort::TaskOverrideRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::TaskOverrideService::TaskOverrideService;

#[derive(Default)]
struct MockTaskOverrideRepository {
    rows: Mutex<Vec<TaskOverride>>,
}

#[async_trait]
impl TaskOverrideRepositoryPort for MockTaskOverrideRepository {
    async fn find_by_project(&self, project_id: Uuid) -> Result<Vec<TaskOverride>, DomainError> {
        Ok(self.rows.lock().unwrap().iter().filter(|r| r.project_id == project_id).cloned().collect())
    }

    async fn find_by_project_and_ticket(&self, project_id: Uuid, ticket_key: &str) -> Result<Option<TaskOverride>, DomainError> {
        Ok(self.rows.lock().unwrap().iter().find(|r| r.project_id == project_id && r.ticket_key == ticket_key).cloned())
    }

    async fn save(&self, task_override: &TaskOverride) -> Result<(), DomainError> {
        self.rows.lock().unwrap().push(task_override.clone());
        Ok(())
    }

    async fn update(&self, task_override: &TaskOverride) -> Result<(), DomainError> {
        let mut rows = self.rows.lock().unwrap();
        if let Some(existing) = rows.iter_mut().find(|r| r.id == task_override.id) {
            *existing = task_override.clone();
        }
        Ok(())
    }
}

#[tokio::test]
async fn set_summary_override_creates_a_new_row_the_first_time() {
    let service = TaskOverrideService::new(Arc::new(MockTaskOverrideRepository::default()));
    let project_id = Uuid::new_v4();

    let response = service.set_summary_override(project_id, "SCRUM-1".into(), "New summary".into()).await.unwrap();

    assert_eq!(response.ticket_key, "SCRUM-1");
    assert_eq!(response.summary, "New summary");

    let overrides = service.find_overrides_by_project(project_id).await.unwrap();
    assert_eq!(overrides.len(), 1);
}

#[tokio::test]
async fn set_summary_override_updates_the_existing_row_for_the_same_ticket() {
    let service = TaskOverrideService::new(Arc::new(MockTaskOverrideRepository::default()));
    let project_id = Uuid::new_v4();

    let first = service.set_summary_override(project_id, "SCRUM-1".into(), "First edit".into()).await.unwrap();
    let second = service.set_summary_override(project_id, "SCRUM-1".into(), "Second edit".into()).await.unwrap();

    assert_eq!(first.id, second.id);
    assert_eq!(second.summary, "Second edit");

    let overrides = service.find_overrides_by_project(project_id).await.unwrap();
    assert_eq!(overrides.len(), 1);
    assert_eq!(overrides[0].summary, "Second edit");
}

#[tokio::test]
async fn set_summary_override_fails_when_summary_is_empty() {
    let service = TaskOverrideService::new(Arc::new(MockTaskOverrideRepository::default()));
    let result = service.set_summary_override(Uuid::new_v4(), "SCRUM-1".into(), "   ".into()).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn find_overrides_by_project_only_returns_that_projects_rows() {
    let service = TaskOverrideService::new(Arc::new(MockTaskOverrideRepository::default()));
    let project_a = Uuid::new_v4();
    let project_b = Uuid::new_v4();

    service.set_summary_override(project_a, "SCRUM-1".into(), "A's ticket".into()).await.unwrap();
    service.set_summary_override(project_b, "SCRUM-2".into(), "B's ticket".into()).await.unwrap();

    let overrides = service.find_overrides_by_project(project_a).await.unwrap();
    assert_eq!(overrides.len(), 1);
    assert_eq!(overrides[0].ticket_key, "SCRUM-1");
}
