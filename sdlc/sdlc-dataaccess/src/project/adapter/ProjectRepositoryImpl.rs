use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::Project::Project;
use sdlc_domain::port::output::ProjectRepositoryPort::ProjectRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::project::mapper::ProjectDataMapper::ProjectDataMapper;
use crate::project::repository::ProjectSeaOrmRepository::ProjectSeaOrmRepository;

pub struct ProjectRepositoryImpl {
    sea_orm_repo: ProjectSeaOrmRepository,
}

impl ProjectRepositoryImpl {
    pub fn new(sea_orm_repo: ProjectSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl ProjectRepositoryPort for ProjectRepositoryImpl {
    async fn save(&self, project: &Project) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(ProjectDataMapper::to_active_model(project)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save project"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, project: &Project) -> Result<(), DomainError> {
        self.sea_orm_repo.update(ProjectDataMapper::to_active_model(project)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update project"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Project>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(ProjectDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find project"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Project>, DomainError> {
        self.sea_orm_repo.find_by_tenant(tenant_id).await
            .map(|models| models.iter().map(ProjectDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list projects"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_slug_and_tenant(&self, slug: &str, tenant_id: Uuid) -> Result<Option<Project>, DomainError> {
        self.sea_orm_repo.find_by_slug_and_tenant(slug, tenant_id).await
            .map(|opt| opt.as_ref().map(ProjectDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find project by slug"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_all_with_jira_tool(&self) -> Result<Vec<Project>, DomainError> {
        self.sea_orm_repo.find_all_with_jira_tool().await
            .map(|models| models.iter().map(ProjectDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list projects with a Jira tool"); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(error = %e, "Failed to delete project"); DomainError::InternalError(e.to_string()) })
    }
}
