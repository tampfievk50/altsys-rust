use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::r#enum::DomainError::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SdlcRunStatus {
    Running,
    Completed,
    Failed,
}

impl fmt::Display for SdlcRunStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SdlcRunStatus::Running => "running",
            SdlcRunStatus::Completed => "completed",
            SdlcRunStatus::Failed => "failed",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for SdlcRunStatus {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "running" => Ok(Self::Running),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            other => Err(DomainError::InternalError(format!("Unknown SDLC run status: {}", other))),
        }
    }
}
