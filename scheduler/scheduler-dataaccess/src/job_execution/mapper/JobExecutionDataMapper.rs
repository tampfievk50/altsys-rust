use sea_orm::Set;
use scheduler_domain::dto::JobExecution::JobExecution;
use crate::job_execution::entity::JobExecutionEntity;

pub struct JobExecutionDataMapper;

impl JobExecutionDataMapper {
    pub fn to_domain(model: &JobExecutionEntity::Model) -> JobExecution {
        JobExecution {
            id: model.id,
            scheduler_id: model.scheduler_id,
            trigger_type: model.trigger_type.clone(),
            status: model.status.clone(),
            started_at: model.started_at,
            finished_at: model.finished_at,
            status_code: model.status_code,
            response_body: model.response_body.clone(),
            error_message: model.error_message.clone(),
        }
    }

    pub fn to_active_model(execution: &JobExecution) -> JobExecutionEntity::ActiveModel {
        JobExecutionEntity::ActiveModel {
            id: Set(execution.id),
            scheduler_id: Set(execution.scheduler_id),
            trigger_type: Set(execution.trigger_type.clone()),
            status: Set(execution.status.clone()),
            started_at: Set(execution.started_at),
            finished_at: Set(execution.finished_at),
            status_code: Set(execution.status_code),
            response_body: Set(execution.response_body.clone()),
            error_message: Set(execution.error_message.clone()),
        }
    }
}
