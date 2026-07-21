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

use crate::dto::CreateCredentialCommand::CreateCredentialCommand;
use crate::dto::Credential::{Credential, NewCredential};
use crate::dto::CredentialResponse::CredentialResponse;
use crate::dto::CredentialSecretResponse::CredentialSecretResponse;
use crate::dto::UpdateCredentialCommand::UpdateCredentialCommand;
use crate::port::input::CredentialPort::CredentialPort;
use crate::port::output::CredentialRepositoryPort::CredentialRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct CredentialService {
    credential_repository: Arc<dyn CredentialRepositoryPort>,
}

impl CredentialService {
    pub fn new(credential_repository: Arc<dyn CredentialRepositoryPort>) -> Self {
        Self { credential_repository }
    }

    fn cipher() -> Result<Aes256Gcm, DomainError> {
        let key_b64 = env::var("CREDENTIAL_ENCRYPTION_KEY")
            .map_err(|_| DomainError::InternalError("CREDENTIAL_ENCRYPTION_KEY must be set".into()))?;
        let key_bytes = STANDARD.decode(key_b64)
            .map_err(|e| DomainError::InternalError(format!("Invalid CREDENTIAL_ENCRYPTION_KEY: {}", e)))?;
        if key_bytes.len() != 32 {
            return Err(DomainError::InternalError("CREDENTIAL_ENCRYPTION_KEY must decode to 32 bytes".into()));
        }
        Ok(Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes)))
    }

    fn encrypt_secret(plaintext: &str) -> Result<String, DomainError> {
        let cipher = Self::cipher()?;
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, plaintext.as_bytes())
            .map_err(|e| DomainError::InternalError(format!("Encryption failed: {}", e)))?;
        let mut combined = nonce.to_vec();
        combined.extend_from_slice(&ciphertext);
        Ok(STANDARD.encode(combined))
    }

    fn decrypt_secret(encoded: &str) -> Result<String, DomainError> {
        let cipher = Self::cipher()?;
        let combined = STANDARD.decode(encoded)
            .map_err(|e| DomainError::InternalError(format!("Failed to decode secret: {}", e)))?;
        if combined.len() < 12 {
            return Err(DomainError::InternalError("Invalid encrypted secret".into()));
        }
        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let plaintext = cipher.decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
            .map_err(|e| DomainError::InternalError(format!("Decryption failed: {}", e)))?;
        String::from_utf8(plaintext).map_err(|e| DomainError::InternalError(format!("Invalid UTF-8 in decrypted secret: {}", e)))
    }

    fn secret_hint(plaintext: &str) -> Option<String> {
        if plaintext.len() <= 4 {
            return None;
        }
        Some(format!("****{}", &plaintext[plaintext.len() - 4..]))
    }

    fn to_response(credential: &Credential) -> CredentialResponse {
        CredentialResponse {
            id: credential.id,
            tenant_id: credential.tenant_id,
            name: credential.name.clone(),
            provider: credential.provider.clone(),
            secret_hint: credential.secret_hint.clone(),
            metadata: credential.metadata.clone(),
            is_active: credential.is_active,
            created_at: credential.created_at,
            updated_at: credential.updated_at,
            created_by: credential.created_by,
            updated_by: credential.updated_by,
        }
    }
}

#[async_trait]
impl CredentialPort for CredentialService {
    async fn create_credential(&self, command: CreateCredentialCommand) -> Result<CredentialResponse, DomainError> {
        info!(name = %command.name, "Creating credential");
        if command.name.trim().is_empty() {
            return Err(DomainError::ValidationError("Credential name cannot be empty".into()));
        }
        if command.provider.trim().is_empty() {
            return Err(DomainError::ValidationError("Provider cannot be empty".into()));
        }
        if command.secret.trim().is_empty() {
            return Err(DomainError::ValidationError("Secret cannot be empty".into()));
        }
        if self.credential_repository.find_by_name_and_tenant(&command.name, command.tenant_id).await?.is_some() {
            return Err(DomainError::AlreadyExists(format!("Credential '{}' already exists in this tenant", command.name)));
        }
        let encrypted_secret = Self::encrypt_secret(&command.secret)?;
        let secret_hint = Self::secret_hint(&command.secret);
        let credential = Credential::new(NewCredential {
            tenant_id: command.tenant_id,
            name: command.name,
            provider: command.provider,
            encrypted_secret,
            secret_hint,
            metadata: command.metadata,
        });
        self.credential_repository.save(&credential).await?;
        info!(credential_id = %credential.id, "Credential created");
        Ok(Self::to_response(&credential))
    }

    async fn find_credential_by_id(&self, id: Uuid) -> Result<CredentialResponse, DomainError> {
        let credential = self.credential_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Credential not found: {}", id)))?;
        Ok(Self::to_response(&credential))
    }

    async fn find_credentials_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<CredentialResponse>, DomainError> {
        let credentials = self.credential_repository.find_by_tenant(tenant_id).await?;
        Ok(credentials.iter().map(Self::to_response).collect())
    }

    async fn update_credential(&self, id: Uuid, command: UpdateCredentialCommand) -> Result<CredentialResponse, DomainError> {
        info!(credential_id = %id, "Updating credential");
        let mut credential = self.credential_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Credential not found: {}", id)))?;
        if let Some(name) = command.name {
            credential.name = name;
        }
        if let Some(secret) = command.secret {
            if secret.trim().is_empty() {
                return Err(DomainError::ValidationError("Secret cannot be empty".into()));
            }
            credential.encrypted_secret = Self::encrypt_secret(&secret)?;
            credential.secret_hint = Self::secret_hint(&secret);
        }
        if let Some(metadata) = command.metadata {
            credential.metadata = Some(metadata);
        }
        if let Some(is_active) = command.is_active {
            credential.is_active = is_active;
        }
        credential.updated_at = Utc::now();
        self.credential_repository.update(&credential).await?;
        Ok(Self::to_response(&credential))
    }

    async fn delete_credential(&self, id: Uuid) -> Result<(), DomainError> {
        info!(credential_id = %id, "Deleting credential");
        let deleted = self.credential_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(credential_id = %id, "Credential not found for deletion");
            return Err(DomainError::NotFound(format!("Credential not found: {}", id)));
        }
        Ok(())
    }

    async fn reveal_credential_secret(&self, id: Uuid) -> Result<CredentialSecretResponse, DomainError> {
        let credential = self.credential_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Credential not found: {}", id)))?;
        let secret = Self::decrypt_secret(&credential.encrypted_secret)?;
        Ok(CredentialSecretResponse {
            id: credential.id,
            provider: credential.provider,
            secret,
        })
    }
}
