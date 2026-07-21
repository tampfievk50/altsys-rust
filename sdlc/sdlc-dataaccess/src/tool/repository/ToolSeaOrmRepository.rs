use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

use crate::tool::entity::ToolEntity;
use crate::tool::entity::ToolEntity::Model;

pub struct ToolSeaOrmRepository {
    db: DatabaseConnection,
}

impl ToolSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, tool: ToolEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        tool.insert(&self.db).await
    }

    pub async fn update(&self, tool: ToolEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        tool.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        ToolEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        ToolEntity::Entity::find()
            .filter(
                Condition::any()
                    .add(ToolEntity::Column::TenantId.eq(tenant_id))
                    .add(ToolEntity::Column::TenantId.is_null()),
            )
            .all(&self.db)
            .await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match ToolEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(tool) => { tool.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }
}
