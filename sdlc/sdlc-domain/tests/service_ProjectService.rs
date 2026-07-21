use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;
use sdlc_domain::dto::CreateProjectCommand::CreateProjectCommand;
use sdlc_domain::dto::Project::{NewProject, Project};
use sdlc_domain::dto::ProjectResponse::ProjectResponse;
use sdlc_domain::dto::Tool::Tool;
use sdlc_domain::dto::UpdateProjectCommand::UpdateProjectCommand;
use sdlc_domain::port::input::ProjectPort::ProjectPort;
use sdlc_domain::port::output::ProjectRepositoryPort::ProjectRepositoryPort;
use sdlc_domain::port::output::ToolRepositoryPort::ToolRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::ProjectService::ProjectService;

use std::sync::Mutex;

#[derive(Default)]
struct MockProjectRepository {
    projects: Mutex<Vec<Project>>,
}

struct MockToolRepository {
    tools: Vec<Tool>,
}

fn github_tool_id() -> Uuid {
    Uuid::from_u128(1)
}

fn jira_tool_id() -> Uuid {
    Uuid::from_u128(2)
}

fn sample_tool(id: Uuid, tool_type: &str) -> Tool {
    let now = Utc::now();
    Tool {
        id,
        tenant_id: None,
        name: format!("{} tool", tool_type),
        tool_type: tool_type.into(),
        description: None,
        config: None,
        is_enabled: true,
        created_at: now,
        updated_at: now,
        created_by: None,
        updated_by: None,
    }
}

impl Default for MockToolRepository {
    fn default() -> Self {
        Self { tools: vec![sample_tool(github_tool_id(), "github"), sample_tool(jira_tool_id(), "jira")] }
    }
}

#[async_trait]
impl ToolRepositoryPort for MockToolRepository {
    async fn save(&self, _tool: &Tool) -> Result<(), DomainError> {
        Ok(())
    }

    async fn update(&self, _tool: &Tool) -> Result<(), DomainError> {
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tool>, DomainError> {
        Ok(self.tools.iter().find(|t| t.id == id).cloned())
    }

    async fn find_by_tenant_including_global(&self, _tenant_id: Uuid) -> Result<Vec<Tool>, DomainError> {
        Ok(self.tools.clone())
    }

    async fn delete_by_id(&self, _id: Uuid) -> Result<bool, DomainError> {
        Ok(true)
    }
}

fn sample_service() -> ProjectService {
    ProjectService::new(Arc::new(MockProjectRepository::default()), Arc::new(MockToolRepository::default()))
}

#[async_trait]
impl ProjectRepositoryPort for MockProjectRepository {
    async fn save(&self, project: &Project) -> Result<(), DomainError> {
        self.projects.lock().unwrap().push(project.clone());
        Ok(())
    }

    async fn update(&self, project: &Project) -> Result<(), DomainError> {
        let mut projects = self.projects.lock().unwrap();
        if let Some(existing) = projects.iter_mut().find(|p| p.id == project.id) {
            *existing = project.clone();
        }
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Project>, DomainError> {
        Ok(self.projects.lock().unwrap().iter().find(|p| p.id == id).cloned())
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Project>, DomainError> {
        Ok(self.projects.lock().unwrap().iter().filter(|p| p.tenant_id == tenant_id).cloned().collect())
    }

    async fn find_by_slug_and_tenant(&self, slug: &str, tenant_id: Uuid) -> Result<Option<Project>, DomainError> {
        Ok(self.projects.lock().unwrap().iter().find(|p| p.slug == slug && p.tenant_id == tenant_id).cloned())
    }

    async fn find_all_with_jira_tool(&self) -> Result<Vec<Project>, DomainError> {
        Ok(self.projects.lock().unwrap().iter().filter(|p| p.jira_tool_id.is_some()).cloned().collect())
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        let mut projects = self.projects.lock().unwrap();
        let len_before = projects.len();
        projects.retain(|p| p.id != id);
        Ok(projects.len() != len_before)
    }
}

fn sample_command(tenant_id: Uuid) -> CreateProjectCommand {
    CreateProjectCommand {
        tenant_id,
        name: "AI Platform".into(),
        slug: "ai-platform".into(),
        github_tool_id: github_tool_id(),
        default_branch: None,
        jira_tool_id: Some(jira_tool_id()),
        build_command: Some("cargo build".into()),
        test_command: Some("cargo test".into()),
        coding_standards: None,
        workflow_config: None,
    }
}

#[tokio::test]
async fn create_project_succeeds_and_defaults_branch_to_main() {
    let service = sample_service();
    let tenant_id = Uuid::new_v4();
    let response = service.create_project(sample_command(tenant_id)).await.unwrap();
    assert_eq!(response.default_branch, "main");
    assert_eq!(response.slug, "ai-platform");
    assert!(response.is_active);
}

#[tokio::test]
async fn create_project_fails_when_name_is_empty() {
    let service = sample_service();
    let mut command = sample_command(Uuid::new_v4());
    command.name = "".into();
    let result = service.create_project(command).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn create_project_fails_when_slug_already_exists_in_tenant() {
    let service = sample_service();
    let tenant_id = Uuid::new_v4();
    service.create_project(sample_command(tenant_id)).await.unwrap();
    let result = service.create_project(sample_command(tenant_id)).await;
    assert!(matches!(result, Err(DomainError::AlreadyExists(_))));
}

#[tokio::test]
async fn find_project_by_id_returns_not_found_for_unknown_id() {
    let service = sample_service();
    let result = service.find_project_by_id(Uuid::new_v4()).await;
    assert!(matches!(result, Err(DomainError::NotFound(_))));
}

#[tokio::test]
async fn update_project_applies_partial_changes() {
    let service = sample_service();
    let created = service.create_project(sample_command(Uuid::new_v4())).await.unwrap();
    let updated = service.update_project(created.id, UpdateProjectCommand {
        name: Some("Renamed".into()),
        github_tool_id: None,
        default_branch: None,
        jira_tool_id: None,
        build_command: None,
        test_command: None,
        coding_standards: None,
        workflow_config: None,
        is_active: Some(false),
    }).await.unwrap();
    assert_eq!(updated.name, "Renamed");
    assert!(!updated.is_active);
}

#[tokio::test]
async fn delete_project_fails_when_not_found() {
    let service = sample_service();
    let result = service.delete_project(Uuid::new_v4()).await;
    assert!(matches!(result, Err(DomainError::NotFound(_))));
}
