use sea_orm::Set;

use sdlc_domain::dto::WorkflowDefinition::WorkflowDefinition;

use crate::workflow_definition::entity::WorkflowDefinitionEntity;

pub struct WorkflowDefinitionDataMapper;

impl WorkflowDefinitionDataMapper {
    pub fn to_domain(entity: &WorkflowDefinitionEntity::Model) -> WorkflowDefinition {
        WorkflowDefinition {
            id: entity.id,
            tenant_id: entity.tenant_id,
            key: entity.key.clone(),
            version: entity.version,
            name: entity.name.clone(),
            description: entity.description.clone(),
            definition: entity.definition.clone(),
            is_active: entity.is_active,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            created_by: entity.created_by,
            updated_by: entity.updated_by,
        }
    }

    pub fn to_active_model(definition: &WorkflowDefinition) -> WorkflowDefinitionEntity::ActiveModel {
        WorkflowDefinitionEntity::ActiveModel {
            id: Set(definition.id),
            tenant_id: Set(definition.tenant_id),
            key: Set(definition.key.clone()),
            version: Set(definition.version),
            name: Set(definition.name.clone()),
            description: Set(definition.description.clone()),
            definition: Set(definition.definition.clone()),
            is_active: Set(definition.is_active),
            created_at: Set(definition.created_at),
            updated_at: Set(definition.updated_at),
            created_by: Set(definition.created_by),
            updated_by: Set(definition.updated_by),
        }
    }
}
