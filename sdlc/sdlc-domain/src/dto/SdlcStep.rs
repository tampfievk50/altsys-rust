use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::r#enum::DomainError::DomainError;

/// The fixed pipeline from the Phase 6 roadmap diagram: Jira Ticket → Load Project
/// Context → Retrieve Organizational Knowledge → Planner Agent → Architecture Review →
/// Create Git Branch → Developer Agent → Compile (fix loop) → Run Tests (fix loop) →
/// Reviewer Agent → Generate Documentation → Commit Changes → Push Branch →
/// Create Pull Request → Update Jira → Done.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SdlcStep {
    FetchTicket,
    LoadProjectContext,
    RetrieveKnowledge,
    PlannerAgent,
    ArchitectureReview,
    CreateGitBranch,
    DeveloperAgent,
    Compile,
    RunTests,
    ReviewerAgent,
    GenerateDocumentation,
    CommitChanges,
    PushBranch,
    CreatePullRequest,
    UpdateJira,
}

impl fmt::Display for SdlcStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SdlcStep::FetchTicket => "fetch_ticket",
            SdlcStep::LoadProjectContext => "load_project_context",
            SdlcStep::RetrieveKnowledge => "retrieve_knowledge",
            SdlcStep::PlannerAgent => "planner_agent",
            SdlcStep::ArchitectureReview => "architecture_review",
            SdlcStep::CreateGitBranch => "create_git_branch",
            SdlcStep::DeveloperAgent => "developer_agent",
            SdlcStep::Compile => "compile",
            SdlcStep::RunTests => "run_tests",
            SdlcStep::ReviewerAgent => "reviewer_agent",
            SdlcStep::GenerateDocumentation => "generate_documentation",
            SdlcStep::CommitChanges => "commit_changes",
            SdlcStep::PushBranch => "push_branch",
            SdlcStep::CreatePullRequest => "create_pull_request",
            SdlcStep::UpdateJira => "update_jira",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for SdlcStep {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fetch_ticket" => Ok(Self::FetchTicket),
            "load_project_context" => Ok(Self::LoadProjectContext),
            "retrieve_knowledge" => Ok(Self::RetrieveKnowledge),
            "planner_agent" => Ok(Self::PlannerAgent),
            "architecture_review" => Ok(Self::ArchitectureReview),
            "create_git_branch" => Ok(Self::CreateGitBranch),
            "developer_agent" => Ok(Self::DeveloperAgent),
            "compile" => Ok(Self::Compile),
            "run_tests" => Ok(Self::RunTests),
            "reviewer_agent" => Ok(Self::ReviewerAgent),
            "generate_documentation" => Ok(Self::GenerateDocumentation),
            "commit_changes" => Ok(Self::CommitChanges),
            "push_branch" => Ok(Self::PushBranch),
            "create_pull_request" => Ok(Self::CreatePullRequest),
            "update_jira" => Ok(Self::UpdateJira),
            other => Err(DomainError::InternalError(format!("Unknown SDLC step: {}", other))),
        }
    }
}
