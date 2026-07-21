use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::dto::SdlcStep::SdlcStep;
use crate::dto::StepExecutionStatus::StepExecutionStatus;

#[derive(Debug, Clone)]
pub struct SdlcStepExecution {
    pub id: Uuid,
    pub run_id: Uuid,
    pub step: SdlcStep,
    pub attempt: i32,
    pub status: StepExecutionStatus,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewSdlcStepExecution {
    pub run_id: Uuid,
    pub step: SdlcStep,
    pub attempt: i32,
}

impl SdlcStepExecution {
    pub fn new(fields: NewSdlcStepExecution) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            run_id: fields.run_id,
            step: fields.step,
            attempt: fields.attempt,
            status: StepExecutionStatus::Running,
            output: None,
            error: None,
            started_at: Some(now),
            completed_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}
