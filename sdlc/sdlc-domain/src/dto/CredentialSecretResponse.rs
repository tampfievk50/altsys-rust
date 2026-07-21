use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Only returned by the explicit reveal-secret use case, never by list/get.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CredentialSecretResponse {
    pub id: Uuid,
    pub provider: String,
    pub secret: String,
}
