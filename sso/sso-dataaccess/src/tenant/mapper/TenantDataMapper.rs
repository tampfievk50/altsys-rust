use sea_orm::Set;
use sso_domain::dto::Tenant::Tenant;
use crate::tenant::entity::TenantEntity;

pub struct TenantDataMapper;

impl TenantDataMapper {
    pub fn to_domain(model: &TenantEntity::Model) -> Tenant {
        Tenant {
            id: model.id,
            name: model.name.clone(),
            slug: model.slug.clone(),
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
        }
    }

    pub fn to_active_model(tenant: &Tenant) -> TenantEntity::ActiveModel {
        TenantEntity::ActiveModel {
            id: Set(tenant.id),
            name: Set(tenant.name.clone()),
            slug: Set(tenant.slug.clone()),
            is_active: Set(tenant.is_active),
            created_at: Set(tenant.created_at),
            updated_at: Set(tenant.updated_at),
            created_by: Set(tenant.created_by),
            updated_by: Set(tenant.updated_by),
        }
    }
}
