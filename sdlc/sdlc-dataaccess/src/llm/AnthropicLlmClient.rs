use async_trait::async_trait;
use rig_core::client::CompletionClient;
use rig_core::completion::Prompt;
use rig_core::providers::anthropic;

use sdlc_domain::dto::LlmCompletionRequest::LlmCompletionRequest;
use sdlc_domain::dto::LlmCompletionResponse::LlmCompletionResponse;
use sdlc_domain::port::output::LlmClientPort::LlmClientPort;
use sdlc_domain::r#enum::DomainError::DomainError;

/// Real "Rig integration" for Anthropic (`provider: "anthropic"`). Reads
/// `ANTHROPIC_API_KEY` from the environment lazily on each call, so a missing
/// key only fails calls that actually use this provider rather than the whole
/// service at startup. `ANTHROPIC_API_BASE_URL`, if set, points the client at
/// a local/self-hosted Anthropic-compatible endpoint instead of the real
/// Anthropic API — the model name in each Agent's config still selects
/// whatever model that endpoint serves.
pub struct AnthropicLlmClient;

#[async_trait]
impl LlmClientPort for AnthropicLlmClient {
    fn provider_name(&self) -> &'static str {
        "anthropic"
    }

    async fn complete(&self, request: LlmCompletionRequest) -> Result<LlmCompletionResponse, DomainError> {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| DomainError::InternalError("ANTHROPIC_API_KEY must be set".into()))?;
        let mut builder = anthropic::Client::builder().api_key(api_key);
        if let Ok(base_url) = std::env::var("ANTHROPIC_API_BASE_URL") {
            builder = builder.base_url(base_url);
        }
        let client = builder.build()
            .map_err(|e| DomainError::InternalError(format!("Failed to build Anthropic client: {}", e)))?;

        let mut builder = client.agent(&request.model).preamble(&request.system_prompt);
        if let Some(temperature) = request.temperature {
            builder = builder.temperature(temperature as f64);
        }
        let agent = builder.build();

        let text = agent.prompt(request.user_prompt.as_str()).await
            .map_err(|e| DomainError::InternalError(format!("Anthropic completion failed: {}", e)))?;

        Ok(LlmCompletionResponse { text })
    }
}
