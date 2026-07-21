use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRunResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}
