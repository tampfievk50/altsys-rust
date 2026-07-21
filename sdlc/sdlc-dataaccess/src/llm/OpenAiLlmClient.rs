use async_trait::async_trait;
use rig_core::client::{CompletionClient, ProviderClient};
use rig_core::completion::Prompt;
use rig_core::providers::openai;

use sdlc_domain::dto::LlmCompletionRequest::LlmCompletionRequest;
use sdlc_domain::dto::LlmCompletionResponse::LlmCompletionResponse;
use sdlc_domain::port::output::LlmClientPort::LlmClientPort;
use sdlc_domain::r#enum::DomainError::DomainError;

/// Real "Rig integration" for OpenAI (`provider: "openai"`). Reads `OPENAI_API_KEY`
/// from the environment lazily on each call (via `rig_core`'s `Client::from_env`),
/// so a missing key only fails calls that actually use this provider rather than
/// the whole service at startup.
pub struct OpenAiLlmClient;

#[async_trait]
impl LlmClientPort for OpenAiLlmClient {
    fn provider_name(&self) -> &'static str {
        "openai"
    }

    async fn complete(&self, request: LlmCompletionRequest) -> Result<LlmCompletionResponse, DomainError> {
        let client = openai::Client::from_env()
            .map_err(|e| DomainError::InternalError(format!("Failed to build OpenAI client: {}", e)))?;

        let mut builder = client.agent(&request.model).preamble(&request.system_prompt);
        if let Some(temperature) = request.temperature {
            builder = builder.temperature(temperature as f64);
        }
        let agent = builder.build();

        let text = agent.prompt(request.user_prompt.as_str()).await
            .map_err(|e| DomainError::InternalError(format!("OpenAI completion failed: {}", e)))?;

        Ok(LlmCompletionResponse { text })
    }
}
