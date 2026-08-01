use std::str::FromStr;

use chrono::{DateTime, Utc};
use cron::Schedule;

use crate::r#enum::DomainError::DomainError;

pub struct CronUtil;

impl CronUtil {
    pub fn validate(expression: &str) -> Result<(), DomainError> {
        Schedule::from_str(expression)
            .map(|_| ())
            .map_err(|e| DomainError::ValidationError(format!("Invalid cron expression: {}", e)))
    }

    pub fn next_fire_time(expression: &str, after: DateTime<Utc>) -> Result<DateTime<Utc>, DomainError> {
        let schedule = Schedule::from_str(expression)
            .map_err(|e| DomainError::ValidationError(format!("Invalid cron expression: {}", e)))?;
        schedule
            .after(&after)
            .next()
            .ok_or_else(|| DomainError::ValidationError("Cron expression has no future occurrences".into()))
    }
}
