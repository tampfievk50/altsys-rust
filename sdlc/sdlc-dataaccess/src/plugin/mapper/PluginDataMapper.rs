use sea_orm::Set;

use sdlc_domain::dto::Plugin::Plugin;

use crate::plugin::entity::PluginEntity;

pub struct PluginDataMapper;

impl PluginDataMapper {
    pub fn to_domain(entity: &PluginEntity::Model) -> Plugin {
        Plugin {
            id: entity.id,
            tenant_id: entity.tenant_id,
            name: entity.name.clone(),
            webhook_url: entity.webhook_url.clone(),
            secret: entity.secret.clone(),
            is_active: entity.is_active,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            created_by: entity.created_by,
            updated_by: entity.updated_by,
        }
    }

    pub fn to_active_model(plugin: &Plugin) -> PluginEntity::ActiveModel {
        PluginEntity::ActiveModel {
            id: Set(plugin.id),
            tenant_id: Set(plugin.tenant_id),
            name: Set(plugin.name.clone()),
            webhook_url: Set(plugin.webhook_url.clone()),
            secret: Set(plugin.secret.clone()),
            is_active: Set(plugin.is_active),
            created_at: Set(plugin.created_at),
            updated_at: Set(plugin.updated_at),
            created_by: Set(plugin.created_by),
            updated_by: Set(plugin.updated_by),
        }
    }
}
