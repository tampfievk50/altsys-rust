use async_trait::async_trait;
use uuid::Uuid;

use crate::r#enum::DomainError::DomainError;

/// Driven adapter for the Workflow service (Phase 4).
#[async_trait]
pub trait WorkflowClientPort: Send + Sync {
    /// Returns the started execution as JSON (the workflow service's own response body).
    async fn start_execution(&self, workflow_definition_id: Uuid, tenant_id: Uuid, context: serde_json::Value) -> Result<serde_json::Value, DomainError>;
    /// Resolves a `WorkflowDefinition.key` to the tenant's latest version's ID,
    /// for callers (e.g. `ActionSpec::ClassifyAndDispatch`) that only know the key.
    async fn find_definition_id_by_key(&self, tenant_id: Uuid, key: &str) -> Result<Uuid, DomainError>;
}
