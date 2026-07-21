use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteAgentCommand {
    pub tenant_id: Uuid,
    /// The task/context given to the agent's LLM call as the user prompt.
    pub input: String,
}
