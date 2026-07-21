use sea_orm::Set;

use sdlc_domain::dto::Prompt::Prompt;

use crate::prompt::entity::PromptEntity;

pub struct PromptDataMapper;

impl PromptDataMapper {
    pub fn to_domain(model: &PromptEntity::Model) -> Prompt {
        Prompt {
            id: model.id,
            tenant_id: model.tenant_id,
            key: model.key.clone(),
            version: model.version,
            content: model.content.clone(),
            variables: model.variables.clone(),
            description: model.description.clone(),
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
        }
    }

    pub fn to_active_model(prompt: &Prompt) -> PromptEntity::ActiveModel {
        PromptEntity::ActiveModel {
            id: Set(prompt.id),
            tenant_id: Set(prompt.tenant_id),
            key: Set(prompt.key.clone()),
            version: Set(prompt.version),
            content: Set(prompt.content.clone()),
            variables: Set(prompt.variables.clone()),
            description: Set(prompt.description.clone()),
            is_active: Set(prompt.is_active),
            created_at: Set(prompt.created_at),
            updated_at: Set(prompt.updated_at),
            created_by: Set(prompt.created_by),
            updated_by: Set(prompt.updated_by),
        }
    }
}
