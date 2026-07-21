use async_trait::async_trait;

use crate::r#enum::DomainError::DomainError;

/// Driven adapter for the Autonomous SDLC service (Phase 6).
#[async_trait]
pub trait SdlcClientPort: Send + Sync {
    /// `parameters` is forwarded verbatim as the `start_run` request body, so the
    /// automation rule's author is responsible for supplying every field
    /// `StartSdlcRunCommand` requires (tenant_id, project_id, agent ids, tool ids, ...).
    async fn start_run(&self, parameters: serde_json::Value) -> Result<serde_json::Value, DomainError>;
}
