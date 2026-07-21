use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sdlc_domain::dto::ApprovalDecisionCommand::ApprovalDecisionCommand;
use sdlc_domain::dto::StartWorkflowExecutionCommand::StartWorkflowExecutionCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct StartWorkflowExecutionRequest {
    pub tenant_id: Uuid,
    pub workflow_definition_id: Uuid,
    pub context: Option<String>,
}

impl From<StartWorkflowExecutionRequest> for StartWorkflowExecutionCommand {
    fn from(val: StartWorkflowExecutionRequest) -> Self {
        StartWorkflowExecutionCommand {
            tenant_id: val.tenant_id,
            workflow_definition_id: val.workflow_definition_id,
            context: val.context,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ApprovalDecisionRequest {
    pub approved: bool,
    pub comment: Option<String>,
}

impl From<ApprovalDecisionRequest> for ApprovalDecisionCommand {
    fn from(val: ApprovalDecisionRequest) -> Self {
        ApprovalDecisionCommand {
            approved: val.approved,
            comment: val.comment,
        }
    }
}
