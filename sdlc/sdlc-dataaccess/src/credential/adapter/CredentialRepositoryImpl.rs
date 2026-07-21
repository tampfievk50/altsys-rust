use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::Credential::Credential;
use sdlc_domain::port::output::CredentialRepositoryPort::CredentialRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::credential::mapper::CredentialDataMapper::CredentialDataMapper;
use crate::credential::repository::CredentialSeaOrmRepository::CredentialSeaOrmRepository;

pub struct CredentialRepositoryImpl {
    sea_orm_repo: CredentialSeaOrmRepository,
}

impl CredentialRepositoryImpl {
    pub fn new(sea_orm_repo: CredentialSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl CredentialRepositoryPort for CredentialRepositoryImpl {
    async fn save(&self, credential: &Credential) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(CredentialDataMapper::to_active_model(credential)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save credential"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, credential: &Credential) -> Result<(), DomainError> {
        self.sea_orm_repo.update(CredentialDataMapper::to_active_model(credential)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update credential"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Credential>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(CredentialDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find credential"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Credential>, DomainError> {
        self.sea_orm_repo.find_by_tenant(tenant_id).await
            .map(|models| models.iter().map(CredentialDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list credentials"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_name_and_tenant(&self, name: &str, tenant_id: Uuid) -> Result<Option<Credential>, DomainError> {
        self.sea_orm_repo.find_by_name_and_tenant(name, tenant_id).await
            .map(|opt| opt.as_ref().map(CredentialDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find credential by name"); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(error = %e, "Failed to delete credential"); DomainError::InternalError(e.to_string()) })
    }
}
