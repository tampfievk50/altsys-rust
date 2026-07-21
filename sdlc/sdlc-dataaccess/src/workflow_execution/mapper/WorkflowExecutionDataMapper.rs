use std::str::FromStr;

use sea_orm::Set;

use sdlc_domain::dto::ExecutionStatus::ExecutionStatus;
use sdlc_domain::dto::WorkflowExecution::WorkflowExecution;

use crate::workflow_execution::entity::WorkflowExecutionEntity;

pub struct WorkflowExecutionDataMapper;

impl WorkflowExecutionDataMapper {
    pub fn to_domain(entity: &WorkflowExecutionEntity::Model) -> WorkflowExecution {
        WorkflowExecution {
            id: entity.id,
            tenant_id: entity.tenant_id,
            workflow_definition_id: entity.workflow_definition_id,
            status: ExecutionStatus::from_str(&entity.status).unwrap_or(ExecutionStatus::Failed),
            context: serde_json::from_str(&entity.context).unwrap_or_else(|_| serde_json::json!({})),
            error: entity.error.clone(),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            started_at: entity.started_at,
            completed_at: entity.completed_at,
            created_by: entity.created_by,
            updated_by: entity.updated_by,
        }
    }

    pub fn to_active_model(execution: &WorkflowExecution) -> WorkflowExecutionEntity::ActiveModel {
        WorkflowExecutionEntity::ActiveModel {
            id: Set(execution.id),
            tenant_id: Set(execution.tenant_id),
            workflow_definition_id: Set(execution.workflow_definition_id),
            status: Set(execution.status.to_string()),
            context: Set(execution.context.to_string()),
            error: Set(execution.error.clone()),
            started_at: Set(execution.started_at),
            completed_at: Set(execution.completed_at),
            created_at: Set(execution.created_at),
            updated_at: Set(execution.updated_at),
            created_by: Set(execution.created_by),
            updated_by: Set(execution.updated_by),
        }
    }
}
