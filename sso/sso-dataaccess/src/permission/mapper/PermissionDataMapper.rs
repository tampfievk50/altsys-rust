use sea_orm::Set;
use sso_domain::dto::Permission::Permission;
use crate::permission::entity::PermissionEntity;

pub struct PermissionDataMapper;

impl PermissionDataMapper {
    pub fn to_domain(model: &PermissionEntity::Model) -> Permission {
        Permission {
            id: model.id,
            name: model.name.clone(),
            action: model.action.clone(),
            resource: model.resource.clone(),
            description: model.description.clone(),
            created_at: model.created_at,
            updated_at: model.updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
        }
    }

    pub fn to_active_model(p: &Permission) -> PermissionEntity::ActiveModel {
        PermissionEntity::ActiveModel {
            id: Set(p.id),
            name: Set(p.name.clone()),
            action: Set(p.action.clone()),
            resource: Set(p.resource.clone()),
            description: Set(p.description.clone()),
            created_at: Set(p.created_at),
            updated_at: Set(p.updated_at),
            created_by: Set(p.created_by),
            updated_by: Set(p.updated_by),
        }
    }
}
