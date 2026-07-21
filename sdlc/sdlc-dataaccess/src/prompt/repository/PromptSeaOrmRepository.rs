use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

use crate::prompt::entity::PromptEntity;
use crate::prompt::entity::PromptEntity::Model;

pub struct PromptSeaOrmRepository {
    db: DatabaseConnection,
}

impl PromptSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: PromptEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: PromptEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        PromptEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_all_by_key_and_tenant(&self, tenant_id: Uuid, key: &str) -> Result<Vec<Model>, sea_orm::DbErr> {
        PromptEntity::Entity::find()
            .filter(PromptEntity::Column::TenantId.eq(tenant_id))
            .filter(PromptEntity::Column::Key.eq(key))
            .all(&self.db)
            .await
    }

    pub async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        PromptEntity::Entity::find()
            .filter(PromptEntity::Column::TenantId.eq(tenant_id))
            .all(&self.db)
            .await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match PromptEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(model) => { model.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }
}
