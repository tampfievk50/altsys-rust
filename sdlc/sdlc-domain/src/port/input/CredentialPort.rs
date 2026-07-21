use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::CreateCredentialCommand::CreateCredentialCommand;
use crate::dto::CredentialResponse::CredentialResponse;
use crate::dto::CredentialSecretResponse::CredentialSecretResponse;
use crate::dto::UpdateCredentialCommand::UpdateCredentialCommand;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait CredentialPort: Send + Sync {
    async fn create_credential(&self, command: CreateCredentialCommand) -> Result<CredentialResponse, DomainError>;
    async fn find_credential_by_id(&self, id: Uuid) -> Result<CredentialResponse, DomainError>;
    async fn find_credentials_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<CredentialResponse>, DomainError>;
    async fn update_credential(&self, id: Uuid, command: UpdateCredentialCommand) -> Result<CredentialResponse, DomainError>;
    async fn delete_credential(&self, id: Uuid) -> Result<(), DomainError>;
    async fn reveal_credential_secret(&self, id: Uuid) -> Result<CredentialSecretResponse, DomainError>;
}
