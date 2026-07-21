use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::dto::NodeExecutionStatus::NodeExecutionStatus;

#[derive(Debug, Clone)]
pub struct WorkflowNodeExecution {
    pub id: Uuid,
    pub workflow_execution_id: Uuid,
    pub node_id: String,
    pub status: NodeExecutionStatus,
    pub attempt: i32,
    pub input: Option<serde_json::Value>,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewWorkflowNodeExecution {
    pub workflow_execution_id: Uuid,
    pub node_id: String,
    pub status: NodeExecutionStatus,
    pub attempt: i32,
    pub input: Option<serde_json::Value>,
}

impl WorkflowNodeExecution {
    pub fn new(fields: NewWorkflowNodeExecution) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            workflow_execution_id: fields.workflow_execution_id,
            node_id: fields.node_id,
            status: fields.status,
            attempt: fields.attempt,
            input: fields.input,
            output: None,
            error: None,
            started_at: Some(now),
            completed_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}
