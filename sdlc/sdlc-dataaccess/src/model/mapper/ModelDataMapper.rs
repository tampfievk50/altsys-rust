use sea_orm::Set;

use sdlc_domain::dto::Model::Model;

use crate::model::entity::ModelEntity;

pub struct ModelDataMapper;

impl ModelDataMapper {
    pub fn to_domain(entity: &ModelEntity::Model) -> Model {
        Model {
            id: entity.id,
            tenant_id: entity.tenant_id,
            provider: entity.provider.clone(),
            model_name: entity.model_name.clone(),
            capability: entity.capability.clone(),
            credential_id: entity.credential_id,
            endpoint_url: entity.endpoint_url.clone(),
            is_active: entity.is_active,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            created_by: entity.created_by,
            updated_by: entity.updated_by,
        }
    }

    pub fn to_active_model(model: &Model) -> ModelEntity::ActiveModel {
        ModelEntity::ActiveModel {
            id: Set(model.id),
            tenant_id: Set(model.tenant_id),
            provider: Set(model.provider.clone()),
            model_name: Set(model.model_name.clone()),
            capability: Set(model.capability.clone()),
            credential_id: Set(model.credential_id),
            endpoint_url: Set(model.endpoint_url.clone()),
            is_active: Set(model.is_active),
            created_at: Set(model.created_at),
            updated_at: Set(model.updated_at),
            created_by: Set(model.created_by),
            updated_by: Set(model.updated_by),
        }
    }
}
