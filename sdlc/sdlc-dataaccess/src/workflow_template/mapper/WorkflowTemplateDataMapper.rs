use sea_orm::Set;

use sdlc_domain::dto::WorkflowTemplate::WorkflowTemplate;

use crate::workflow_template::entity::WorkflowTemplateEntity;

pub struct WorkflowTemplateDataMapper;

impl WorkflowTemplateDataMapper {
    pub fn to_domain(entity: &WorkflowTemplateEntity::Model) -> WorkflowTemplate {
        WorkflowTemplate {
            id: entity.id,
            tenant_id: entity.tenant_id,
            key: entity.key.clone(),
            version: entity.version,
            name: entity.name.clone(),
            description: entity.description.clone(),
            definition_template: entity.definition_template.clone(),
            is_active: entity.is_active,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            created_by: entity.created_by,
            updated_by: entity.updated_by,
        }
    }

    pub fn to_active_model(template: &WorkflowTemplate) -> WorkflowTemplateEntity::ActiveModel {
        WorkflowTemplateEntity::ActiveModel {
            id: Set(template.id),
            tenant_id: Set(template.tenant_id),
            key: Set(template.key.clone()),
            version: Set(template.version),
            name: Set(template.name.clone()),
            description: Set(template.description.clone()),
            definition_template: Set(template.definition_template.clone()),
            is_active: Set(template.is_active),
            created_at: Set(template.created_at),
            updated_at: Set(template.updated_at),
            created_by: Set(template.created_by),
            updated_by: Set(template.updated_by),
        }
    }
}
