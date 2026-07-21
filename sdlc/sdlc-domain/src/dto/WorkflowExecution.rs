use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::dto::ExecutionStatus::ExecutionStatus;

/// Kept as a parsed `serde_json::Value` (not JSON text) because the engine reads
/// and merges into it on every node completion; the dataaccess mapper is the only
/// place that (de)serializes it to/from the `context` TEXT column.
#[derive(Debug, Clone)]
pub struct WorkflowExecution {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub workflow_definition_id: Uuid,
    pub status: ExecutionStatus,
    pub context: serde_json::Value,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

pub struct NewWorkflowExecution {
    pub tenant_id: Uuid,
    pub workflow_definition_id: Uuid,
    pub context: serde_json::Value,
}

impl WorkflowExecution {
    pub fn new(fields: NewWorkflowExecution) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: fields.tenant_id,
            workflow_definition_id: fields.workflow_definition_id,
            status: ExecutionStatus::Running,
            context: fields.context,
            error: None,
            created_at: now,
            updated_at: now,
            started_at: Some(now),
            completed_at: None,
            created_by: None,
            updated_by: None,
        }
    }

    /// Shallow-merges a node's JSON output into the execution context so later edge
    /// conditions can reference its fields directly (e.g. `tests_passed == true`).
    pub fn merge_context(&mut self, output: &serde_json::Value) {
        if let (Some(base), Some(incoming)) = (self.context.as_object_mut(), output.as_object()) {
            for (key, value) in incoming {
                base.insert(key.clone(), value.clone());
            }
        }
    }
}
