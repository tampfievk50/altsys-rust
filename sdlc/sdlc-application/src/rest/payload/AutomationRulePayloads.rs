use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sdlc_domain::dto::CreateAutomationRuleCommand::CreateAutomationRuleCommand;
use sdlc_domain::dto::UpdateAutomationRuleCommand::UpdateAutomationRuleCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAutomationRuleRequest {
    pub tenant_id: Uuid,
    pub name: String,
    pub event_type: String,
    pub match_criteria: Option<String>,
    pub action: String,
}

impl From<CreateAutomationRuleRequest> for CreateAutomationRuleCommand {
    fn from(val: CreateAutomationRuleRequest) -> Self {
        CreateAutomationRuleCommand {
            tenant_id: val.tenant_id,
            name: val.name,
            event_type: val.event_type,
            match_criteria: val.match_criteria,
            action: val.action,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateAutomationRuleRequest {
    pub name: Option<String>,
    pub match_criteria: Option<String>,
    pub action: Option<String>,
    pub is_active: Option<bool>,
}

impl From<UpdateAutomationRuleRequest> for UpdateAutomationRuleCommand {
    fn from(val: UpdateAutomationRuleRequest) -> Self {
        UpdateAutomationRuleCommand {
            name: val.name,
            match_criteria: val.match_criteria,
            action: val.action,
            is_active: val.is_active,
        }
    }
}
