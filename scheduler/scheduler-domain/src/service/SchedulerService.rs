use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::CreateSchedulerCommand::CreateSchedulerCommand;
use crate::dto::Scheduler::Scheduler;
use crate::dto::SchedulerResponse::SchedulerResponse;
use crate::dto::UpdateSchedulerCommand::UpdateSchedulerCommand;
use crate::port::input::SchedulerPort::SchedulerPort;
use crate::port::output::SchedulerRepositoryPort::SchedulerRepositoryPort;
use crate::r#enum::DomainError::DomainError;
use crate::util::CronUtil::CronUtil;

const ALLOWED_METHODS: [&str; 5] = ["GET", "POST", "PUT", "PATCH", "DELETE"];
pub const JOB_TYPE_WEBHOOK: &str = "WEBHOOK";
pub const JOB_TYPE_COMMAND: &str = "COMMAND";

pub struct SchedulerService {
    scheduler_repository: Arc<dyn SchedulerRepositoryPort>,
}

impl SchedulerService {
    pub fn new(scheduler_repository: Arc<dyn SchedulerRepositoryPort>) -> Self {
        Self { scheduler_repository }
    }

    fn to_response(scheduler: &Scheduler) -> SchedulerResponse {
        SchedulerResponse {
            id: scheduler.id,
            name: scheduler.name.clone(),
            description: scheduler.description.clone(),
            job_type: scheduler.job_type.clone(),
            cron_expression: scheduler.cron_expression.clone(),
            webhook_url: scheduler.webhook_url.clone(),
            http_method: scheduler.http_method.clone(),
            headers: scheduler.headers.clone(),
            body: scheduler.body.clone(),
            timeout_seconds: scheduler.timeout_seconds,
            command_line: scheduler.command_line.clone(),
            working_dir: scheduler.working_dir.clone(),
            detached: scheduler.detached,
            last_pid: scheduler.last_pid,
            is_active: scheduler.is_active,
            next_run_at: scheduler.next_run_at,
            last_run_at: scheduler.last_run_at,
            created_at: scheduler.created_at,
            updated_at: scheduler.updated_at,
        }
    }

    fn normalize_method(method: &str) -> Result<String, DomainError> {
        let upper = method.trim().to_uppercase();
        if !ALLOWED_METHODS.contains(&upper.as_str()) {
            return Err(DomainError::ValidationError(format!(
                "Unsupported HTTP method: {}",
                method
            )));
        }
        Ok(upper)
    }

    fn normalize_job_type(job_type: &str) -> Result<String, DomainError> {
        let upper = job_type.trim().to_uppercase();
        if upper != JOB_TYPE_WEBHOOK && upper != JOB_TYPE_COMMAND {
            return Err(DomainError::ValidationError(format!(
                "Unsupported job_type: {} (expected WEBHOOK or COMMAND)",
                job_type
            )));
        }
        Ok(upper)
    }

    fn validate_working_dir(dir: &str) -> Result<(), DomainError> {
        if !Path::new(dir).is_dir() {
            return Err(DomainError::ValidationError(format!("working_dir does not exist: {}", dir)));
        }
        Ok(())
    }

    async fn load(&self, id: Uuid) -> Result<Scheduler, DomainError> {
        self.scheduler_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Scheduler not found: {}", id)))
    }
}

#[async_trait]
impl SchedulerPort for SchedulerService {
    async fn create_scheduler(&self, command: CreateSchedulerCommand) -> Result<SchedulerResponse, DomainError> {
        info!(name = %command.name, job_type = %command.job_type, "Creating scheduler");

        if command.name.trim().is_empty() {
            return Err(DomainError::ValidationError("Scheduler name cannot be empty".into()));
        }
        let job_type = Self::normalize_job_type(&command.job_type)?;
        CronUtil::validate(&command.cron_expression)?;

        let timeout_seconds = command.timeout_seconds.unwrap_or(30);
        if timeout_seconds <= 0 {
            return Err(DomainError::ValidationError("timeout_seconds must be positive".into()));
        }

        let (webhook_url, http_method, command_line, working_dir, detached) = match job_type.as_str() {
            JOB_TYPE_WEBHOOK => {
                let webhook_url = command
                    .webhook_url
                    .filter(|s| !s.trim().is_empty())
                    .ok_or_else(|| DomainError::ValidationError("webhook_url is required for job_type WEBHOOK".into()))?;
                let http_method = Self::normalize_method(
                    command
                        .http_method
                        .as_deref()
                        .ok_or_else(|| DomainError::ValidationError("http_method is required for job_type WEBHOOK".into()))?,
                )?;
                (Some(webhook_url), Some(http_method), None, None, false)
            }
            JOB_TYPE_COMMAND => {
                let command_line = command
                    .command_line
                    .filter(|s| !s.trim().is_empty())
                    .ok_or_else(|| DomainError::ValidationError("command_line is required for job_type COMMAND".into()))?;
                if let Some(dir) = &command.working_dir {
                    Self::validate_working_dir(dir)?;
                }
                (None, None, Some(command_line), command.working_dir, command.detached.unwrap_or(false))
            }
            _ => unreachable!("normalize_job_type already validated job_type"),
        };

        let next_run_at = Some(CronUtil::next_fire_time(&command.cron_expression, Utc::now())?);

        let scheduler = Scheduler::new(
            command.name,
            command.description,
            job_type,
            command.cron_expression,
            webhook_url,
            http_method,
            command.headers,
            command.body,
            timeout_seconds,
            command_line,
            working_dir,
            detached,
            next_run_at,
        );

        self.scheduler_repository.save(&scheduler).await?;
        info!(scheduler_id = %scheduler.id, "Scheduler created");
        Ok(Self::to_response(&scheduler))
    }

    async fn find_scheduler_by_id(&self, id: Uuid) -> Result<SchedulerResponse, DomainError> {
        let scheduler = self.load(id).await?;
        Ok(Self::to_response(&scheduler))
    }

    async fn find_all_schedulers(&self) -> Result<Vec<SchedulerResponse>, DomainError> {
        let schedulers = self.scheduler_repository.find_all().await?;
        Ok(schedulers.iter().map(Self::to_response).collect())
    }

    async fn update_scheduler(&self, id: Uuid, command: UpdateSchedulerCommand) -> Result<SchedulerResponse, DomainError> {
        info!(scheduler_id = %id, "Updating scheduler");
        let mut scheduler = self.load(id).await?;

        if let Some(name) = command.name {
            if name.trim().is_empty() {
                return Err(DomainError::ValidationError("Scheduler name cannot be empty".into()));
            }
            scheduler.name = name;
        }
        if let Some(description) = command.description {
            scheduler.description = Some(description);
        }
        if let Some(timeout_seconds) = command.timeout_seconds {
            if timeout_seconds <= 0 {
                return Err(DomainError::ValidationError("timeout_seconds must be positive".into()));
            }
            scheduler.timeout_seconds = timeout_seconds;
        }
        if let Some(cron_expression) = command.cron_expression {
            CronUtil::validate(&cron_expression)?;
            scheduler.next_run_at = Some(CronUtil::next_fire_time(&cron_expression, Utc::now())?);
            scheduler.cron_expression = cron_expression;
        }

        match scheduler.job_type.as_str() {
            JOB_TYPE_WEBHOOK => {
                if command.command_line.is_some() || command.working_dir.is_some() || command.detached.is_some() {
                    return Err(DomainError::ValidationError("Cannot set command fields on a WEBHOOK scheduler".into()));
                }
                if let Some(webhook_url) = command.webhook_url {
                    if webhook_url.trim().is_empty() {
                        return Err(DomainError::ValidationError("Webhook URL cannot be empty".into()));
                    }
                    scheduler.webhook_url = Some(webhook_url);
                }
                if let Some(http_method) = command.http_method {
                    scheduler.http_method = Some(Self::normalize_method(&http_method)?);
                }
                if let Some(headers) = command.headers {
                    scheduler.headers = Some(headers);
                }
                if let Some(body) = command.body {
                    scheduler.body = Some(body);
                }
            }
            JOB_TYPE_COMMAND => {
                if command.webhook_url.is_some() || command.http_method.is_some() {
                    return Err(DomainError::ValidationError("Cannot set webhook fields on a COMMAND scheduler".into()));
                }
                if let Some(command_line) = command.command_line {
                    if command_line.trim().is_empty() {
                        return Err(DomainError::ValidationError("command_line cannot be empty".into()));
                    }
                    scheduler.command_line = Some(command_line);
                }
                if let Some(working_dir) = command.working_dir {
                    Self::validate_working_dir(&working_dir)?;
                    scheduler.working_dir = Some(working_dir);
                }
                if let Some(detached) = command.detached {
                    scheduler.detached = detached;
                }
            }
            _ => unreachable!("job_type is validated at creation time"),
        }

        scheduler.updated_at = Utc::now();
        self.scheduler_repository.update(&scheduler).await?;
        Ok(Self::to_response(&scheduler))
    }

    async fn delete_scheduler(&self, id: Uuid) -> Result<(), DomainError> {
        info!(scheduler_id = %id, "Deleting scheduler");
        let deleted = self.scheduler_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(scheduler_id = %id, "Scheduler not found for deletion");
            return Err(DomainError::NotFound(format!("Scheduler not found: {}", id)));
        }
        Ok(())
    }

    async fn pause_scheduler(&self, id: Uuid) -> Result<SchedulerResponse, DomainError> {
        let mut scheduler = self.load(id).await?;
        scheduler.is_active = false;
        scheduler.updated_at = Utc::now();
        self.scheduler_repository.update(&scheduler).await?;
        info!(scheduler_id = %id, "Scheduler paused");
        Ok(Self::to_response(&scheduler))
    }

    async fn resume_scheduler(&self, id: Uuid) -> Result<SchedulerResponse, DomainError> {
        let mut scheduler = self.load(id).await?;
        scheduler.is_active = true;
        scheduler.next_run_at = Some(CronUtil::next_fire_time(&scheduler.cron_expression, Utc::now())?);
        scheduler.updated_at = Utc::now();
        self.scheduler_repository.update(&scheduler).await?;
        info!(scheduler_id = %id, "Scheduler resumed");
        Ok(Self::to_response(&scheduler))
    }
}
