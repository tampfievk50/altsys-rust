use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::r#enum::DomainError::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepExecutionStatus {
    Running,
    Succeeded,
    Failed,
}

impl fmt::Display for StepExecutionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            StepExecutionStatus::Running => "running",
            StepExecutionStatus::Succeeded => "succeeded",
            StepExecutionStatus::Failed => "failed",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for StepExecutionStatus {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "running" => Ok(Self::Running),
            "succeeded" => Ok(Self::Succeeded),
            "failed" => Ok(Self::Failed),
            other => Err(DomainError::InternalError(format!("Unknown step execution status: {}", other))),
        }
    }
}
