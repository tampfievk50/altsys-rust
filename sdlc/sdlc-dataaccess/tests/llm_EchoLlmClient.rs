use async_trait::async_trait;
use tracing::info;
use sdlc_domain::dto::LlmCompletionRequest::LlmCompletionRequest;
use sdlc_domain::dto::LlmCompletionResponse::LlmCompletionResponse;
use sdlc_domain::port::output::LlmClientPort::LlmClientPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_dataaccess::llm::EchoLlmClient::EchoLlmClient;

#[tokio::test]
async fn complete_echoes_the_prompt() {
    let client = EchoLlmClient;
    let response = client.complete(LlmCompletionRequest {
        system_prompt: "You are a planner.".into(),
        user_prompt: "Plan the release.".into(),
        model: "echo-1".into(),
        temperature: None,
    }).await.unwrap();
    assert_eq!(response.text, "[echo:echo-1] Plan the release.");
}

#[tokio::test]
async fn complete_fails_on_empty_prompt() {
    let client = EchoLlmClient;
    let result = client.complete(LlmCompletionRequest {
        system_prompt: "sys".into(),
        user_prompt: "  ".into(),
        model: "echo-1".into(),
        temperature: None,
    }).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}
