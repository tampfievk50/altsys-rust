use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

use crate::model::entity::ModelEntity;
use crate::model::entity::ModelEntity::Model;

pub struct ModelSeaOrmRepository {
    db: DatabaseConnection,
}

impl ModelSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: ModelEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: ModelEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        ModelEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        ModelEntity::Entity::find()
            .filter(
                Condition::any()
                    .add(ModelEntity::Column::TenantId.eq(tenant_id))
                    .add(ModelEntity::Column::TenantId.is_null()),
            )
            .all(&self.db)
            .await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match ModelEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(model) => { model.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }
}
