use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ToolExecutionResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub exit_code: Option<i32>,
    pub duration_ms: i64,
}

impl ToolExecutionResult {
    pub fn ok(output: String, duration_ms: i64) -> Self {
        Self { success: true, output, error: None, exit_code: Some(0), duration_ms }
    }

    pub fn failed(output: String, error: String, exit_code: Option<i32>, duration_ms: i64) -> Self {
        Self { success: false, output, error: Some(error), exit_code, duration_ms }
    }
}
