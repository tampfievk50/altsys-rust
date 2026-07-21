use std::str::FromStr;

use sea_orm::Set;

use sdlc_domain::dto::NodeExecutionStatus::NodeExecutionStatus;
use sdlc_domain::dto::WorkflowNodeExecution::WorkflowNodeExecution;

use crate::workflow_node_execution::entity::WorkflowNodeExecutionEntity;

pub struct WorkflowNodeExecutionDataMapper;

impl WorkflowNodeExecutionDataMapper {
    pub fn to_domain(entity: &WorkflowNodeExecutionEntity::Model) -> WorkflowNodeExecution {
        WorkflowNodeExecution {
            id: entity.id,
            workflow_execution_id: entity.workflow_execution_id,
            node_id: entity.node_id.clone(),
            status: NodeExecutionStatus::from_str(&entity.status).unwrap_or(NodeExecutionStatus::Failed),
            attempt: entity.attempt,
            input: entity.input.as_deref().and_then(|s| serde_json::from_str(s).ok()),
            output: entity.output.as_deref().and_then(|s| serde_json::from_str(s).ok()),
            error: entity.error.clone(),
            started_at: entity.started_at,
            completed_at: entity.completed_at,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }

    pub fn to_active_model(node_execution: &WorkflowNodeExecution) -> WorkflowNodeExecutionEntity::ActiveModel {
        WorkflowNodeExecutionEntity::ActiveModel {
            id: Set(node_execution.id),
            workflow_execution_id: Set(node_execution.workflow_execution_id),
            node_id: Set(node_execution.node_id.clone()),
            status: Set(node_execution.status.to_string()),
            attempt: Set(node_execution.attempt),
            input: Set(node_execution.input.as_ref().map(|v| v.to_string())),
            output: Set(node_execution.output.as_ref().map(|v| v.to_string())),
            error: Set(node_execution.error.clone()),
            started_at: Set(node_execution.started_at),
            completed_at: Set(node_execution.completed_at),
            created_at: Set(node_execution.created_at),
            updated_at: Set(node_execution.updated_at),
        }
    }
}
