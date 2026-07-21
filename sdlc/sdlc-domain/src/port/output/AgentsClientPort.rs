use async_trait::async_trait;
use uuid::Uuid;

use crate::r#enum::DomainError::DomainError;

/// Driven adapter for the Agents service (Phase 5): runs a Planner/Architect/
/// Developer/Reviewer/Documentation agent and returns its output text.
#[async_trait]
pub trait AgentsClientPort: Send + Sync {
    /// An execution that finishes with status `"failed"` surfaces as `Err`.
    async fn execute_agent(&self, agent_id: Uuid, tenant_id: Uuid, input: String) -> Result<String, DomainError>;
}
