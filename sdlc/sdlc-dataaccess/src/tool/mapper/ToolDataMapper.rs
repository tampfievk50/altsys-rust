use sea_orm::Set;

use sdlc_domain::dto::Tool::Tool;

use crate::tool::entity::ToolEntity;

pub struct ToolDataMapper;

impl ToolDataMapper {
    pub fn to_domain(entity: &ToolEntity::Model) -> Tool {
        Tool {
            id: entity.id,
            tenant_id: entity.tenant_id,
            name: entity.name.clone(),
            tool_type: entity.tool_type.clone(),
            description: entity.description.clone(),
            config: entity.config.clone(),
            is_enabled: entity.is_enabled,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            created_by: entity.created_by,
            updated_by: entity.updated_by,
        }
    }

    pub fn to_active_model(tool: &Tool) -> ToolEntity::ActiveModel {
        ToolEntity::ActiveModel {
            id: Set(tool.id),
            tenant_id: Set(tool.tenant_id),
            name: Set(tool.name.clone()),
            tool_type: Set(tool.tool_type.clone()),
            description: Set(tool.description.clone()),
            config: Set(tool.config.clone()),
            is_enabled: Set(tool.is_enabled),
            created_at: Set(tool.created_at),
            updated_at: Set(tool.updated_at),
            created_by: Set(tool.created_by),
            updated_by: Set(tool.updated_by),
        }
    }
}
