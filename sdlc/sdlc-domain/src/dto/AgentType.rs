use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::r#enum::DomainError::DomainError;

/// The six specialized agent roles that make up the Agent Runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AgentType {
    /// Reads a ticket, retrieves historical knowledge, and produces an implementation
    /// plan (tasks, risks, files to modify, acceptance criteria, complexity estimate).
    Planner,
    /// Reviews the plan against the existing architecture and recommends design decisions.
    Architect,
    /// Implements the planned code changes.
    Developer,
    /// Reviews generated code for standards, security, and architecture violations.
    Reviewer,
    /// Writes or evaluates tests for the change.
    Tester,
    /// Updates README/ADRs/release notes/implementation summaries.
    Documentation,
    /// Reads an incoming task (e.g. a Jira ticket) and decides which workflow
    /// should process it, returning `{"workflow_key": "...", "task_type": "..."}`.
    Classifier,
}

impl fmt::Display for AgentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AgentType::Planner => "planner",
            AgentType::Architect => "architect",
            AgentType::Developer => "developer",
            AgentType::Reviewer => "reviewer",
            AgentType::Tester => "tester",
            AgentType::Documentation => "documentation",
            AgentType::Classifier => "classifier",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for AgentType {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "planner" => Ok(Self::Planner),
            "architect" => Ok(Self::Architect),
            "developer" => Ok(Self::Developer),
            "reviewer" => Ok(Self::Reviewer),
            "tester" => Ok(Self::Tester),
            "documentation" => Ok(Self::Documentation),
            "classifier" => Ok(Self::Classifier),
            other => Err(DomainError::ValidationError(format!(
                "Unknown agent type '{}': expected one of planner, architect, developer, reviewer, tester, documentation, classifier",
                other
            ))),
        }
    }
}
