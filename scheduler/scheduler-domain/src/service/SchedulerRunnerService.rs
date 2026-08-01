use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;
use reqwest::{Client, Method};
use tokio::process::Command as TokioCommand;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::dto::ExecutionResponse::ExecutionResponse;
use crate::dto::JobExecution::JobExecution;
use crate::dto::Scheduler::Scheduler;
use crate::port::input::SchedulerRunnerPort::SchedulerRunnerPort;
use crate::port::output::ExecutionRepositoryPort::ExecutionRepositoryPort;
use crate::port::output::SchedulerRepositoryPort::SchedulerRepositoryPort;
use crate::r#enum::DomainError::DomainError;
use crate::service::SchedulerService::JOB_TYPE_COMMAND;
use crate::util::CronUtil::CronUtil;
use crate::util::ProcessUtil::ProcessUtil;

const MAX_RESPONSE_BODY_LEN: usize = 4000;
const TRIGGER_MANUAL: &str = "MANUAL";
const TRIGGER_SCHEDULED: &str = "SCHEDULED";

fn truncate_utf8_safe(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        return s.to_string();
    }
    let mut end = max_len;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    s[..end].to_string()
}

enum CommandOutcome {
    Skipped { pid: i32 },
    Started { pid: i32, log_path: String },
    Completed { exit_code: i32, output: String },
    TimedOut,
    Error(String),
}

pub struct SchedulerRunnerService {
    scheduler_repository: Arc<dyn SchedulerRepositoryPort>,
    execution_repository: Arc<dyn ExecutionRepositoryPort>,
    http_client: Client,
}

impl SchedulerRunnerService {
    pub fn new(
        scheduler_repository: Arc<dyn SchedulerRepositoryPort>,
        execution_repository: Arc<dyn ExecutionRepositoryPort>,
    ) -> Self {
        Self {
            scheduler_repository,
            execution_repository,
            http_client: Client::new(),
        }
    }

    fn to_response(execution: &JobExecution) -> ExecutionResponse {
        ExecutionResponse {
            id: execution.id,
            scheduler_id: execution.scheduler_id,
            trigger_type: execution.trigger_type.clone(),
            status: execution.status.clone(),
            started_at: execution.started_at,
            finished_at: execution.finished_at,
            status_code: execution.status_code,
            response_body: execution.response_body.clone(),
            error_message: execution.error_message.clone(),
        }
    }

    async fn fire_webhook(&self, scheduler: &Scheduler) -> Result<(u16, Option<String>), String> {
        let webhook_url = scheduler.webhook_url.as_deref().ok_or("Scheduler has no webhook_url configured")?;
        let http_method = scheduler.http_method.as_deref().ok_or("Scheduler has no http_method configured")?;
        let method = Method::from_bytes(http_method.as_bytes())
            .map_err(|e| format!("Invalid HTTP method: {}", e))?;

        let mut request = self
            .http_client
            .request(method, webhook_url)
            .timeout(Duration::from_secs(scheduler.timeout_seconds.max(1) as u64));

        if let Some(headers) = scheduler.headers.as_ref().and_then(|h| h.as_object()) {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request = request.header(key.as_str(), value_str);
                }
            }
        }

        if let Some(body) = &scheduler.body {
            request = request.json(body);
        }

        let response = request.send().await.map_err(|e| e.to_string())?;
        let status = response.status().as_u16();
        let text = response.text().await.unwrap_or_default();
        let response_body = if text.is_empty() {
            None
        } else {
            Some(truncate_utf8_safe(&text, MAX_RESPONSE_BODY_LEN))
        };

        Ok((status, response_body))
    }

    async fn run_command(&self, scheduler: &Scheduler) -> CommandOutcome {
        let command_line = match scheduler.command_line.as_deref() {
            Some(c) if !c.trim().is_empty() => c,
            _ => return CommandOutcome::Error("Scheduler has no command_line configured".into()),
        };

        if let Some(pid) = scheduler.last_pid {
            if ProcessUtil::is_running(pid) {
                return CommandOutcome::Skipped { pid };
            }
        }

        let parts = match shell_words::split(command_line) {
            Ok(p) if !p.is_empty() => p,
            Ok(_) => return CommandOutcome::Error("command_line is empty".into()),
            Err(e) => return CommandOutcome::Error(format!("Failed to parse command_line: {}", e)),
        };

        let mut cmd = TokioCommand::new(&parts[0]);
        cmd.args(&parts[1..]);
        if let Some(dir) = &scheduler.working_dir {
            cmd.current_dir(dir);
        }
        cmd.stdin(Stdio::null());

        if scheduler.detached {
            let log_dir = std::env::var("JOB_LOG_DIR").unwrap_or_else(|_| "./job-logs".into());
            if let Err(e) = std::fs::create_dir_all(&log_dir) {
                return CommandOutcome::Error(format!("Failed to create log dir: {}", e));
            }
            let log_path = format!("{}/{}.log", log_dir, scheduler.id);
            let log_file = match std::fs::OpenOptions::new().create(true).append(true).open(&log_path) {
                Ok(f) => f,
                Err(e) => return CommandOutcome::Error(format!("Failed to open log file: {}", e)),
            };
            let log_file_err = match log_file.try_clone() {
                Ok(f) => f,
                Err(e) => return CommandOutcome::Error(format!("Failed to clone log handle: {}", e)),
            };
            cmd.stdout(Stdio::from(log_file));
            cmd.stderr(Stdio::from(log_file_err));
            // New process group so the child outlives this process/session
            // (equivalent to Popen(..., start_new_session=True) in Python).
            #[cfg(unix)]
            {
                cmd.process_group(0);
            }

            match cmd.spawn() {
                Ok(child) => {
                    let pid = child.id().map(|p| p as i32).unwrap_or(0);
                    // Deliberately not awaited: this is fire-and-forget, the
                    // child keeps running after `child` is dropped here.
                    CommandOutcome::Started { pid, log_path }
                }
                Err(e) => CommandOutcome::Error(format!("Failed to spawn command: {}", e)),
            }
        } else {
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
            cmd.kill_on_drop(true); // if the timeout below fires, the child gets killed

            let timeout = Duration::from_secs(scheduler.timeout_seconds.max(1) as u64);
            match tokio::time::timeout(timeout, cmd.output()).await {
                Ok(Ok(output)) => {
                    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
                    combined.push_str(&String::from_utf8_lossy(&output.stderr));
                    CommandOutcome::Completed {
                        exit_code: output.status.code().unwrap_or(-1),
                        output: truncate_utf8_safe(&combined, MAX_RESPONSE_BODY_LEN),
                    }
                }
                Ok(Err(e)) => CommandOutcome::Error(format!("Failed to run command: {}", e)),
                Err(_) => CommandOutcome::TimedOut,
            }
        }
    }

    async fn execute(&self, scheduler: &Scheduler, trigger_type: &str) -> Result<ExecutionResponse, DomainError> {
        let mut execution = JobExecution::start(scheduler.id, trigger_type);
        self.execution_repository.save(&execution).await?;

        let mut new_last_pid = scheduler.last_pid;

        if scheduler.job_type == JOB_TYPE_COMMAND {
            match self.run_command(scheduler).await {
                CommandOutcome::Skipped { pid } => {
                    execution.status = "SKIPPED".into();
                    execution.response_body = Some(format!("previous run (pid {}) still active", pid));
                }
                CommandOutcome::Started { pid, log_path } => {
                    execution.status = "STARTED".into();
                    execution.response_body = Some(format!("started detached (pid {}), logs: {}", pid, log_path));
                    new_last_pid = Some(pid);
                }
                CommandOutcome::Completed { exit_code, output } => {
                    execution.status_code = Some(exit_code);
                    execution.response_body = Some(output);
                    if exit_code == 0 {
                        execution.status = "SUCCESS".into();
                    } else {
                        execution.status = "FAILED".into();
                        execution.error_message = Some(format!("Command exited with status {}", exit_code));
                    }
                }
                CommandOutcome::TimedOut => {
                    warn!(scheduler_id = %scheduler.id, "Command timed out");
                    execution.status = "FAILED".into();
                    execution.error_message = Some(format!("Command timed out after {}s", scheduler.timeout_seconds));
                }
                CommandOutcome::Error(err) => {
                    warn!(scheduler_id = %scheduler.id, error = %err, "Command execution failed");
                    execution.status = "FAILED".into();
                    execution.error_message = Some(err);
                }
            }
        } else {
            match self.fire_webhook(scheduler).await {
                Ok((status_code, response_body)) => {
                    execution.status_code = Some(status_code as i32);
                    execution.response_body = response_body;
                    if status_code < 400 {
                        execution.status = "SUCCESS".into();
                    } else {
                        execution.status = "FAILED".into();
                        execution.error_message = Some(format!("Webhook responded with status {}", status_code));
                    }
                }
                Err(err) => {
                    warn!(scheduler_id = %scheduler.id, error = %err, "Webhook call failed");
                    execution.status = "FAILED".into();
                    execution.error_message = Some(err);
                }
            }
        }

        execution.finished_at = Some(Utc::now());
        self.execution_repository.update(&execution).await?;

        let mut updated_scheduler = scheduler.clone();
        updated_scheduler.last_run_at = Some(Utc::now());
        updated_scheduler.last_pid = new_last_pid;
        if trigger_type == TRIGGER_SCHEDULED {
            updated_scheduler.next_run_at = CronUtil::next_fire_time(&scheduler.cron_expression, Utc::now()).ok();
        }
        updated_scheduler.updated_at = Utc::now();
        self.scheduler_repository.update(&updated_scheduler).await?;

        Ok(Self::to_response(&execution))
    }
}

#[async_trait]
impl SchedulerRunnerPort for SchedulerRunnerService {
    async fn run_scheduler(&self, id: Uuid) -> Result<ExecutionResponse, DomainError> {
        let scheduler = self
            .scheduler_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Scheduler not found: {}", id)))?;

        info!(scheduler_id = %id, "Manually triggering scheduler");
        self.execute(&scheduler, TRIGGER_MANUAL).await
    }

    async fn tick(&self) -> Result<(), DomainError> {
        let due = self.scheduler_repository.find_due(Utc::now()).await?;
        for scheduler in due {
            let scheduler_id = scheduler.id;
            if let Err(e) = self.execute(&scheduler, TRIGGER_SCHEDULED).await {
                error!(scheduler_id = %scheduler_id, error = %e, "Scheduled execution failed");
            }
        }
        Ok(())
    }
}
