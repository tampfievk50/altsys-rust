use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::EventResponse::EventResponse;
use crate::dto::IngestEventCommand::IngestEventCommand;
use crate::dto::IngestEventResponse::IngestEventResponse;
use crate::dto::RuleFiringResponse::RuleFiringResponse;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait EventPort: Send + Sync {
    /// The automation entry point: persists the event, evaluates every active
    /// `AutomationRule` for the tenant with a matching `event_type`, dispatches the
    /// action for each match, and records one `RuleFiring` per rule evaluated.
    async fn ingest_event(&self, command: IngestEventCommand) -> Result<IngestEventResponse, DomainError>;
    async fn find_events_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<EventResponse>, DomainError>;
    async fn find_firings_by_event(&self, event_id: Uuid) -> Result<Vec<RuleFiringResponse>, DomainError>;
}
