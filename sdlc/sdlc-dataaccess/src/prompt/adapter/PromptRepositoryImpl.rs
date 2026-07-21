use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::Prompt::Prompt;
use sdlc_domain::port::output::PromptRepositoryPort::PromptRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::prompt::mapper::PromptDataMapper::PromptDataMapper;
use crate::prompt::repository::PromptSeaOrmRepository::PromptSeaOrmRepository;

pub struct PromptRepositoryImpl {
    sea_orm_repo: PromptSeaOrmRepository,
}

impl PromptRepositoryImpl {
    pub fn new(sea_orm_repo: PromptSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl PromptRepositoryPort for PromptRepositoryImpl {
    async fn save(&self, prompt: &Prompt) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(PromptDataMapper::to_active_model(prompt)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save prompt"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, prompt: &Prompt) -> Result<(), DomainError> {
        self.sea_orm_repo.update(PromptDataMapper::to_active_model(prompt)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update prompt"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Prompt>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(PromptDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find prompt"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_all_by_key_and_tenant(&self, tenant_id: Uuid, key: &str) -> Result<Vec<Prompt>, DomainError> {
        self.sea_orm_repo.find_all_by_key_and_tenant(tenant_id, key).await
            .map(|models| models.iter().map(PromptDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list prompt versions"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Prompt>, DomainError> {
        self.sea_orm_repo.find_by_tenant(tenant_id).await
            .map(|models| models.iter().map(PromptDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list prompts"); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(error = %e, "Failed to delete prompt"); DomainError::InternalError(e.to_string()) })
    }
}
