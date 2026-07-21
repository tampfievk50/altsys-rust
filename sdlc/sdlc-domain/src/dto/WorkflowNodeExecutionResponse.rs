use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WorkflowNodeExecutionResponse {
    pub id: Uuid,
    pub workflow_execution_id: Uuid,
    pub node_id: String,
    pub status: String,
    pub attempt: i32,
    pub input: Option<String>,
    pub output: Option<String>,
    pub error: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}
