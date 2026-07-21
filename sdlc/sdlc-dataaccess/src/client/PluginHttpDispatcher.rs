use async_trait::async_trait;

use sdlc_domain::dto::Plugin::Plugin;
use sdlc_domain::port::output::PluginDispatchPort::PluginDispatchPort;
use sdlc_domain::r#enum::DomainError::DomainError;

/// The Plugin SDK's invocation mechanism: POSTs `{ event_type, payload, parameters }`
/// to the plugin's registered webhook, with `X-Automation-Secret` set when the
/// plugin was registered with a secret so it can verify the call's origin.
pub struct PluginHttpDispatcher {
    http: reqwest::Client,
}

impl PluginHttpDispatcher {
    pub fn new() -> Self {
        Self { http: reqwest::Client::new() }
    }
}

impl Default for PluginHttpDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PluginDispatchPort for PluginHttpDispatcher {
    async fn dispatch(&self, plugin: &Plugin, event_type: &str, payload: &serde_json::Value, parameters: &serde_json::Value) -> Result<serde_json::Value, DomainError> {
        let mut request = self.http.post(&plugin.webhook_url).json(&serde_json::json!({
            "event_type": event_type,
            "payload": payload,
            "parameters": parameters,
        }));
        if let Some(secret) = &plugin.secret {
            request = request.header("X-Automation-Secret", secret.clone());
        }

        let response = request.send().await
            .map_err(|e| DomainError::InternalError(format!("Failed to reach plugin '{}': {}", plugin.name, e)))?;
        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err(DomainError::InternalError(format!("Plugin '{}' returned {}: {}", plugin.name, status, body)));
        }
        Ok(serde_json::from_str(&body).unwrap_or(serde_json::Value::String(body)))
    }
}
