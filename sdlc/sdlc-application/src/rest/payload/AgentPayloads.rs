use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sdlc_domain::dto::AgentType::AgentType;
use sdlc_domain::dto::CreateAgentCommand::CreateAgentCommand;
use sdlc_domain::dto::UpdateAgentCommand::UpdateAgentCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAgentRequest {
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub agent_type: AgentType,
    pub system_prompt: String,
    pub provider: String,
    pub model: String,
    pub temperature: Option<f32>,
    #[serde(default)]
    pub skill_ids: Vec<Uuid>,
}

impl From<CreateAgentRequest> for CreateAgentCommand {
    fn from(val: CreateAgentRequest) -> Self {
        CreateAgentCommand {
            tenant_id: val.tenant_id,
            name: val.name,
            agent_type: val.agent_type,
            system_prompt: val.system_prompt,
            provider: val.provider,
            model: val.model,
            temperature: val.temperature,
            skill_ids: val.skill_ids,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateAgentRequest {
    pub name: Option<String>,
    pub system_prompt: Option<String>,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub is_active: Option<bool>,
    /// Omit to leave attached skills unchanged; pass an array (possibly empty) to fully replace them.
    pub skill_ids: Option<Vec<Uuid>>,
}

impl From<UpdateAgentRequest> for UpdateAgentCommand {
    fn from(val: UpdateAgentRequest) -> Self {
        UpdateAgentCommand {
            name: val.name,
            system_prompt: val.system_prompt,
            provider: val.provider,
            model: val.model,
            temperature: val.temperature,
            is_active: val.is_active,
            skill_ids: val.skill_ids,
        }
    }
}
