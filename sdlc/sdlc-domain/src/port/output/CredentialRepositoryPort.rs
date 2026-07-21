use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::Credential::Credential;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait CredentialRepositoryPort: Send + Sync {
    async fn save(&self, credential: &Credential) -> Result<(), DomainError>;
    async fn update(&self, credential: &Credential) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Credential>, DomainError>;
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Credential>, DomainError>;
    async fn find_by_name_and_tenant(&self, name: &str, tenant_id: Uuid) -> Result<Option<Credential>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;
}
