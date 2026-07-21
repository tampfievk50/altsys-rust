use std::str::FromStr;

use sea_orm::Set;

use sdlc_domain::dto::AgentExecution::AgentExecution;
use sdlc_domain::dto::AgentExecutionStatus::AgentExecutionStatus;

use crate::agent_execution::entity::AgentExecutionEntity;

pub struct AgentExecutionDataMapper;

impl AgentExecutionDataMapper {
    pub fn to_domain(entity: &AgentExecutionEntity::Model) -> AgentExecution {
        AgentExecution {
            id: entity.id,
            tenant_id: entity.tenant_id,
            agent_id: entity.agent_id,
            input: entity.input.clone(),
            output: entity.output.clone(),
            status: AgentExecutionStatus::from_str(&entity.status).unwrap_or(AgentExecutionStatus::Failed),
            error: entity.error.clone(),
            started_at: entity.started_at,
            completed_at: entity.completed_at,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }

    pub fn to_active_model(execution: &AgentExecution) -> AgentExecutionEntity::ActiveModel {
        AgentExecutionEntity::ActiveModel {
            id: Set(execution.id),
            tenant_id: Set(execution.tenant_id),
            agent_id: Set(execution.agent_id),
            input: Set(execution.input.clone()),
            output: Set(execution.output.clone()),
            status: Set(execution.status.to_string()),
            error: Set(execution.error.clone()),
            started_at: Set(execution.started_at),
            completed_at: Set(execution.completed_at),
            created_at: Set(execution.created_at),
            updated_at: Set(execution.updated_at),
        }
    }
}
