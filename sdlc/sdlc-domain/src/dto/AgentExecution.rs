use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::dto::AgentExecutionStatus::AgentExecutionStatus;

#[derive(Debug, Clone)]
pub struct AgentExecution {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub agent_id: Uuid,
    pub input: String,
    pub output: Option<String>,
    pub status: AgentExecutionStatus,
    pub error: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewAgentExecution {
    pub tenant_id: Uuid,
    pub agent_id: Uuid,
    pub input: String,
}

impl AgentExecution {
    pub fn new(fields: NewAgentExecution) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: fields.tenant_id,
            agent_id: fields.agent_id,
            input: fields.input,
            output: None,
            status: AgentExecutionStatus::Running,
            error: None,
            started_at: Some(now),
            completed_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}
