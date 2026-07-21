use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSearchCommand {
    pub query: String,
    /// Restrict results to one source type (e.g. `adr`, `pull_request`).
    pub source_type: Option<String>,
    /// Defaults to 10, capped at 50.
    pub limit: Option<u32>,
}
