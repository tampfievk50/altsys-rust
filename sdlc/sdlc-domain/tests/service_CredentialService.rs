use std::env;
use std::sync::Arc;
use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use async_trait::async_trait;
use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;
use sdlc_domain::dto::CreateCredentialCommand::CreateCredentialCommand;
use sdlc_domain::dto::Credential::{Credential, NewCredential};
use sdlc_domain::dto::CredentialResponse::CredentialResponse;
use sdlc_domain::dto::CredentialSecretResponse::CredentialSecretResponse;
use sdlc_domain::dto::UpdateCredentialCommand::UpdateCredentialCommand;
use sdlc_domain::port::input::CredentialPort::CredentialPort;
use sdlc_domain::port::output::CredentialRepositoryPort::CredentialRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::CredentialService::CredentialService;

use std::sync::Mutex;

#[derive(Default)]
struct MockCredentialRepository {
    credentials: Mutex<Vec<Credential>>,
}

#[async_trait]
impl CredentialRepositoryPort for MockCredentialRepository {
    async fn save(&self, credential: &Credential) -> Result<(), DomainError> {
        self.credentials.lock().unwrap().push(credential.clone());
        Ok(())
    }

    async fn update(&self, credential: &Credential) -> Result<(), DomainError> {
        let mut credentials = self.credentials.lock().unwrap();
        if let Some(existing) = credentials.iter_mut().find(|c| c.id == credential.id) {
            *existing = credential.clone();
        }
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Credential>, DomainError> {
        Ok(self.credentials.lock().unwrap().iter().find(|c| c.id == id).cloned())
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Credential>, DomainError> {
        Ok(self.credentials.lock().unwrap().iter().filter(|c| c.tenant_id == tenant_id).cloned().collect())
    }

    async fn find_by_name_and_tenant(&self, name: &str, tenant_id: Uuid) -> Result<Option<Credential>, DomainError> {
        Ok(self.credentials.lock().unwrap().iter().find(|c| c.name == name && c.tenant_id == tenant_id).cloned())
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        let mut credentials = self.credentials.lock().unwrap();
        let len_before = credentials.len();
        credentials.retain(|c| c.id != id);
        Ok(credentials.len() != len_before)
    }
}

fn set_test_key() {
    // 32 zero bytes, base64-encoded — deterministic key for tests only.
    std::env::set_var("CREDENTIAL_ENCRYPTION_KEY", "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
}

fn sample_command(tenant_id: Uuid) -> CreateCredentialCommand {
    CreateCredentialCommand {
        tenant_id,
        name: "github-bot".into(),
        provider: "github".into(),
        secret: "ghp_supersecrettoken1234".into(),
        metadata: None,
    }
}

#[tokio::test]
async fn create_credential_never_returns_the_raw_secret() {
    set_test_key();
    let service = CredentialService::new(Arc::new(MockCredentialRepository::default()));
    let response = service.create_credential(sample_command(Uuid::new_v4())).await.unwrap();
    assert_eq!(response.secret_hint.as_deref(), Some("****1234"));
}

#[tokio::test]
async fn create_credential_fails_when_secret_is_empty() {
    set_test_key();
    let service = CredentialService::new(Arc::new(MockCredentialRepository::default()));
    let mut command = sample_command(Uuid::new_v4());
    command.secret = "".into();
    let result = service.create_credential(command).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn create_credential_fails_when_name_already_exists_in_tenant() {
    set_test_key();
    let service = CredentialService::new(Arc::new(MockCredentialRepository::default()));
    let tenant_id = Uuid::new_v4();
    service.create_credential(sample_command(tenant_id)).await.unwrap();
    let result = service.create_credential(sample_command(tenant_id)).await;
    assert!(matches!(result, Err(DomainError::AlreadyExists(_))));
}

#[tokio::test]
async fn reveal_credential_secret_round_trips_the_plaintext() {
    set_test_key();
    let service = CredentialService::new(Arc::new(MockCredentialRepository::default()));
    let created = service.create_credential(sample_command(Uuid::new_v4())).await.unwrap();
    let revealed = service.reveal_credential_secret(created.id).await.unwrap();
    assert_eq!(revealed.secret, "ghp_supersecrettoken1234");
}

#[tokio::test]
async fn reveal_credential_secret_fails_when_not_found() {
    set_test_key();
    let service = CredentialService::new(Arc::new(MockCredentialRepository::default()));
    let result = service.reveal_credential_secret(Uuid::new_v4()).await;
    assert!(matches!(result, Err(DomainError::NotFound(_))));
}

#[tokio::test]
async fn update_credential_re_encrypts_when_secret_changes() {
    set_test_key();
    let service = CredentialService::new(Arc::new(MockCredentialRepository::default()));
    let created = service.create_credential(sample_command(Uuid::new_v4())).await.unwrap();
    service.update_credential(created.id, UpdateCredentialCommand {
        name: None,
        secret: Some("new-secret-value-9999".into()),
        metadata: None,
        is_active: None,
    }).await.unwrap();
    let revealed = service.reveal_credential_secret(created.id).await.unwrap();
    assert_eq!(revealed.secret, "new-secret-value-9999");
}
