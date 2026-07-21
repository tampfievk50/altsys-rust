use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalDecisionCommand {
    pub approved: bool,
    pub comment: Option<String>,
}
