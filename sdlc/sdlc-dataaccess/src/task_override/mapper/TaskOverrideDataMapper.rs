use sea_orm::Set;

use sdlc_domain::dto::TaskOverride::TaskOverride;

use crate::task_override::entity::TaskOverrideEntity;

pub struct TaskOverrideDataMapper;

impl TaskOverrideDataMapper {
    pub fn to_domain(model: &TaskOverrideEntity::Model) -> TaskOverride {
        TaskOverride {
            id: model.id,
            project_id: model.project_id,
            ticket_key: model.ticket_key.clone(),
            summary: model.summary.clone(),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }

    pub fn to_active_model(task_override: &TaskOverride) -> TaskOverrideEntity::ActiveModel {
        TaskOverrideEntity::ActiveModel {
            id: Set(task_override.id),
            project_id: Set(task_override.project_id),
            ticket_key: Set(task_override.ticket_key.clone()),
            summary: Set(task_override.summary.clone()),
            created_at: Set(task_override.created_at),
            updated_at: Set(task_override.updated_at),
        }
    }
}
