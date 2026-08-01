use serde::Deserialize;
use serde_json::Value;
use utoipa::ToSchema;

use scheduler_domain::dto::CreateSchedulerCommand::CreateSchedulerCommand;
use scheduler_domain::dto::UpdateSchedulerCommand::UpdateSchedulerCommand;

/// cron_expression uses the 6/7-field format `sec min hour day-of-month month day-of-week [year]`,
/// e.g. `0 */5 * * * *` fires every 5 minutes. All times are evaluated in UTC.
///
/// job_type is either "WEBHOOK" (requires webhook_url + http_method) or
/// "COMMAND" (requires command_line; working_dir and detached are optional).
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSchedulerRequest {
    pub name: String,
    pub description: Option<String>,
    pub job_type: String,
    pub cron_expression: String,

    pub webhook_url: Option<String>,
    pub http_method: Option<String>,
    pub headers: Option<Value>,
    pub body: Option<Value>,
    pub timeout_seconds: Option<i32>,

    pub command_line: Option<String>,
    pub working_dir: Option<String>,
    pub detached: Option<bool>,
}

impl Into<CreateSchedulerCommand> for CreateSchedulerRequest {
    fn into(self) -> CreateSchedulerCommand {
        CreateSchedulerCommand {
            name: self.name,
            description: self.description,
            job_type: self.job_type,
            cron_expression: self.cron_expression,
            webhook_url: self.webhook_url,
            http_method: self.http_method,
            headers: self.headers,
            body: self.body,
            timeout_seconds: self.timeout_seconds,
            command_line: self.command_line,
            working_dir: self.working_dir,
            detached: self.detached,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSchedulerRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub cron_expression: Option<String>,
    pub timeout_seconds: Option<i32>,

    pub webhook_url: Option<String>,
    pub http_method: Option<String>,
    pub headers: Option<Value>,
    pub body: Option<Value>,

    pub command_line: Option<String>,
    pub working_dir: Option<String>,
    pub detached: Option<bool>,
}

impl Into<UpdateSchedulerCommand> for UpdateSchedulerRequest {
    fn into(self) -> UpdateSchedulerCommand {
        UpdateSchedulerCommand {
            name: self.name,
            description: self.description,
            cron_expression: self.cron_expression,
            timeout_seconds: self.timeout_seconds,
            webhook_url: self.webhook_url,
            http_method: self.http_method,
            headers: self.headers,
            body: self.body,
            command_line: self.command_line,
            working_dir: self.working_dir,
            detached: self.detached,
        }
    }
}
