use std::str::FromStr;

use sea_orm::Set;

use sdlc_domain::dto::SdlcStep::SdlcStep;
use sdlc_domain::dto::SdlcStepExecution::SdlcStepExecution;
use sdlc_domain::dto::StepExecutionStatus::StepExecutionStatus;

use crate::sdlc_step_execution::entity::SdlcStepExecutionEntity;

pub struct SdlcStepExecutionDataMapper;

impl SdlcStepExecutionDataMapper {
    pub fn to_domain(entity: &SdlcStepExecutionEntity::Model) -> SdlcStepExecution {
        SdlcStepExecution {
            id: entity.id,
            run_id: entity.run_id,
            step: SdlcStep::from_str(&entity.step).unwrap_or(SdlcStep::FetchTicket),
            attempt: entity.attempt,
            status: StepExecutionStatus::from_str(&entity.status).unwrap_or(StepExecutionStatus::Failed),
            output: entity.output.as_deref().and_then(|s| serde_json::from_str(s).ok()),
            error: entity.error.clone(),
            started_at: entity.started_at,
            completed_at: entity.completed_at,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }

    pub fn to_active_model(step: &SdlcStepExecution) -> SdlcStepExecutionEntity::ActiveModel {
        SdlcStepExecutionEntity::ActiveModel {
            id: Set(step.id),
            run_id: Set(step.run_id),
            step: Set(step.step.to_string()),
            attempt: Set(step.attempt),
            status: Set(step.status.to_string()),
            output: Set(step.output.as_ref().map(|v| v.to_string())),
            error: Set(step.error.clone()),
            started_at: Set(step.started_at),
            completed_at: Set(step.completed_at),
            created_at: Set(step.created_at),
            updated_at: Set(step.updated_at),
        }
    }
}
