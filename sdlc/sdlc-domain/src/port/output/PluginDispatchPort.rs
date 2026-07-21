use async_trait::async_trait;

use crate::dto::Plugin::Plugin;
use crate::r#enum::DomainError::DomainError;

/// The Plugin SDK's invocation mechanism: posts the event and rule context to a
/// registered `Plugin`'s webhook.
#[async_trait]
pub trait PluginDispatchPort: Send + Sync {
    async fn dispatch(&self, plugin: &Plugin, event_type: &str, payload: &serde_json::Value, parameters: &serde_json::Value) -> Result<serde_json::Value, DomainError>;
}
