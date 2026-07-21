use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

use crate::plugin::entity::PluginEntity;
use crate::plugin::entity::PluginEntity::Model;

pub struct PluginSeaOrmRepository {
    db: DatabaseConnection,
}

impl PluginSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, plugin: PluginEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        plugin.insert(&self.db).await
    }

    pub async fn update(&self, plugin: PluginEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        plugin.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        PluginEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        PluginEntity::Entity::find()
            .filter(
                Condition::any()
                    .add(PluginEntity::Column::TenantId.eq(tenant_id))
                    .add(PluginEntity::Column::TenantId.is_null()),
            )
            .all(&self.db)
            .await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match PluginEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(plugin) => { plugin.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }
}
