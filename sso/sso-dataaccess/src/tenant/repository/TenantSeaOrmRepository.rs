use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

use crate::tenant::entity::TenantEntity;
use crate::tenant::entity::TenantEntity::Model;

pub struct TenantSeaOrmRepository {
    db: DatabaseConnection,
}

impl TenantSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: TenantEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: TenantEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        TenantEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<Model>, sea_orm::DbErr> {
        TenantEntity::Entity::find()
            .filter(TenantEntity::Column::Slug.eq(slug))
            .one(&self.db)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<Model>, sea_orm::DbErr> {
        TenantEntity::Entity::find().all(&self.db).await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match TenantEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(model) => { model.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }
}
