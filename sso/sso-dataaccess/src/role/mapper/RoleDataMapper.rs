use sea_orm::Set;
use sso_domain::dto::Role::Role;
use crate::role::entity::RoleEntity;

pub struct RoleDataMapper;

impl RoleDataMapper {
    pub fn to_domain(model: &RoleEntity::Model) -> Role {
        Role {
            id: model.id,
            tenant_id: model.tenant_id,
            name: model.name.clone(),
            description: model.description.clone(),
            created_at: model.created_at,
            updated_at: model.updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
        }
    }

    pub fn to_active_model(role: &Role) -> RoleEntity::ActiveModel {
        RoleEntity::ActiveModel {
            id: Set(role.id),
            tenant_id: Set(role.tenant_id),
            name: Set(role.name.clone()),
            description: Set(role.description.clone()),
            created_at: Set(role.created_at),
            updated_at: Set(role.updated_at),
            created_by: Set(role.created_by),
            updated_by: Set(role.updated_by),
        }
    }
}
