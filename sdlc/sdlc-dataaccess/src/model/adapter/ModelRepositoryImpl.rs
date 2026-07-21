use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::Model::Model;
use sdlc_domain::port::output::ModelRepositoryPort::ModelRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::model::mapper::ModelDataMapper::ModelDataMapper;
use crate::model::repository::ModelSeaOrmRepository::ModelSeaOrmRepository;

pub struct ModelRepositoryImpl {
    sea_orm_repo: ModelSeaOrmRepository,
}

impl ModelRepositoryImpl {
    pub fn new(sea_orm_repo: ModelSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl ModelRepositoryPort for ModelRepositoryImpl {
    async fn save(&self, model: &Model) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(ModelDataMapper::to_active_model(model)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save model"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, model: &Model) -> Result<(), DomainError> {
        self.sea_orm_repo.update(ModelDataMapper::to_active_model(model)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update model"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(ModelDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find model"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Model>, DomainError> {
        self.sea_orm_repo.find_by_tenant_including_global(tenant_id).await
            .map(|models| models.iter().map(ModelDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list models"); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(error = %e, "Failed to delete model"); DomainError::InternalError(e.to_string()) })
    }
}
