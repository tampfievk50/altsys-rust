use async_trait::async_trait;
use tracing::info;

use sdlc_domain::dto::LlmCompletionRequest::LlmCompletionRequest;
use sdlc_domain::dto::LlmCompletionResponse::LlmCompletionResponse;
use sdlc_domain::port::output::LlmClientPort::LlmClientPort;
use sdlc_domain::r#enum::DomainError::DomainError;

/// Default `LlmClientPort` (`provider: "echo"`). Deterministic and dependency-free —
/// echoes the prompt back instead of calling a real model — so the Agent Runtime
/// works fully offline with no API key. Register `Agent`s against `"openai"` or
/// `"anthropic"` (see `OpenAiLlmClient` / `AnthropicLlmClient`) to use a real model
/// via the Rig integration.
pub struct EchoLlmClient;

#[async_trait]
impl LlmClientPort for EchoLlmClient {
    fn provider_name(&self) -> &'static str {
        "echo"
    }

    async fn complete(&self, request: LlmCompletionRequest) -> Result<LlmCompletionResponse, DomainError> {
        if request.user_prompt.trim().is_empty() {
            return Err(DomainError::ValidationError("Cannot complete an empty prompt".into()));
        }
        info!(model = %request.model, "Completing prompt with the echo provider");
        Ok(LlmCompletionResponse {
            text: format!("[echo:{}] {}", request.model, request.user_prompt),
        })
    }
}
