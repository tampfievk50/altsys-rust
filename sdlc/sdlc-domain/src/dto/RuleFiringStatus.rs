use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::r#enum::DomainError::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleFiringStatus {
    /// The rule's `match_criteria` did not match this event; the action was not run.
    Skipped,
    Succeeded,
    Failed,
}

impl fmt::Display for RuleFiringStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RuleFiringStatus::Skipped => "skipped",
            RuleFiringStatus::Succeeded => "succeeded",
            RuleFiringStatus::Failed => "failed",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for RuleFiringStatus {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "skipped" => Ok(Self::Skipped),
            "succeeded" => Ok(Self::Succeeded),
            "failed" => Ok(Self::Failed),
            other => Err(DomainError::InternalError(format!("Unknown rule firing status: {}", other))),
        }
    }
}
