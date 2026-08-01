use sea_orm::Set;
use scheduler_domain::dto::Scheduler::Scheduler;
use crate::scheduler::entity::SchedulerEntity;

pub struct SchedulerDataMapper;

impl SchedulerDataMapper {
    pub fn to_domain(model: &SchedulerEntity::Model) -> Scheduler {
        Scheduler {
            id: model.id,
            name: model.name.clone(),
            description: model.description.clone(),
            job_type: model.job_type.clone(),
            cron_expression: model.cron_expression.clone(),
            webhook_url: model.webhook_url.clone(),
            http_method: model.http_method.clone(),
            headers: model.headers.clone(),
            body: model.body.clone(),
            timeout_seconds: model.timeout_seconds,
            command_line: model.command_line.clone(),
            working_dir: model.working_dir.clone(),
            detached: model.detached,
            last_pid: model.last_pid,
            is_active: model.is_active,
            next_run_at: model.next_run_at,
            last_run_at: model.last_run_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
        }
    }

    pub fn to_active_model(scheduler: &Scheduler) -> SchedulerEntity::ActiveModel {
        SchedulerEntity::ActiveModel {
            id: Set(scheduler.id),
            name: Set(scheduler.name.clone()),
            description: Set(scheduler.description.clone()),
            job_type: Set(scheduler.job_type.clone()),
            cron_expression: Set(scheduler.cron_expression.clone()),
            webhook_url: Set(scheduler.webhook_url.clone()),
            http_method: Set(scheduler.http_method.clone()),
            headers: Set(scheduler.headers.clone()),
            body: Set(scheduler.body.clone()),
            timeout_seconds: Set(scheduler.timeout_seconds),
            command_line: Set(scheduler.command_line.clone()),
            working_dir: Set(scheduler.working_dir.clone()),
            detached: Set(scheduler.detached),
            last_pid: Set(scheduler.last_pid),
            is_active: Set(scheduler.is_active),
            next_run_at: Set(scheduler.next_run_at),
            last_run_at: Set(scheduler.last_run_at),
            created_at: Set(scheduler.created_at),
            updated_at: Set(scheduler.updated_at),
            created_by: Set(scheduler.created_by),
            updated_by: Set(scheduler.updated_by),
        }
    }
}
