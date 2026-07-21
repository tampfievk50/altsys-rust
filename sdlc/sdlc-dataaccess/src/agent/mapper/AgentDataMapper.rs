use std::str::FromStr;

use sea_orm::Set;

use sdlc_domain::dto::Agent::Agent;
use sdlc_domain::dto::AgentType::AgentType;

use crate::agent::entity::AgentEntity;

pub struct AgentDataMapper;

impl AgentDataMapper {
    pub fn to_domain(entity: &AgentEntity::Model) -> Agent {
        Agent {
            id: entity.id,
            tenant_id: entity.tenant_id,
            name: entity.name.clone(),
            agent_type: AgentType::from_str(&entity.agent_type).unwrap_or(AgentType::Developer),
            system_prompt: entity.system_prompt.clone(),
            provider: entity.provider.clone(),
            model: entity.model.clone(),
            temperature: entity.temperature,
            is_active: entity.is_active,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            created_by: entity.created_by,
            updated_by: entity.updated_by,
        }
    }

    pub fn to_active_model(agent: &Agent) -> AgentEntity::ActiveModel {
        AgentEntity::ActiveModel {
            id: Set(agent.id),
            tenant_id: Set(agent.tenant_id),
            name: Set(agent.name.clone()),
            agent_type: Set(agent.agent_type.to_string()),
            system_prompt: Set(agent.system_prompt.clone()),
            provider: Set(agent.provider.clone()),
            model: Set(agent.model.clone()),
            temperature: Set(agent.temperature),
            is_active: Set(agent.is_active),
            created_at: Set(agent.created_at),
            updated_at: Set(agent.updated_at),
            created_by: Set(agent.created_by),
            updated_by: Set(agent.updated_by),
        }
    }
}
