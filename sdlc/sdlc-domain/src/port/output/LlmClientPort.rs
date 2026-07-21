use async_trait::async_trait;

use crate::dto::LlmCompletionRequest::LlmCompletionRequest;
use crate::dto::LlmCompletionResponse::LlmCompletionResponse;
use crate::r#enum::DomainError::DomainError;

/// A driven adapter for one LLM provider. Implementations live in
/// `agents-dataaccess/src/llm/` — the real ones (`OpenAiLlmClient`, `AnthropicLlmClient`)
/// wrap `rig-core` ("Rig integration"); `EchoLlmClient` is a dependency-free default
/// that needs no API key, so the runtime works fully offline out of the box.
#[async_trait]
pub trait LlmClientPort: Send + Sync {
    /// Matches `Agent::provider` in the registry.
    fn provider_name(&self) -> &'static str;

    async fn complete(&self, request: LlmCompletionRequest) -> Result<LlmCompletionResponse, DomainError>;
}
