#[derive(Debug, Clone)]
pub struct LlmCompletionRequest {
    pub system_prompt: String,
    pub user_prompt: String,
    pub model: String,
    pub temperature: Option<f32>,
}
