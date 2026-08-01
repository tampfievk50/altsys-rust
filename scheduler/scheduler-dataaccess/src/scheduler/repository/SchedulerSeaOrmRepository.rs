use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

use crate::scheduler::entity::SchedulerEntity;
use crate::scheduler::entity::SchedulerEntity::Model;

pub struct SchedulerSeaOrmRepository {
    db: DatabaseConnection,
}

impl SchedulerSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: SchedulerEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: SchedulerEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        SchedulerEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_all(&self) -> Result<Vec<Model>, sea_orm::DbErr> {
        SchedulerEntity::Entity::find().all(&self.db).await
    }

    pub async fn find_due(&self, now: DateTime<Utc>) -> Result<Vec<Model>, sea_orm::DbErr> {
        SchedulerEntity::Entity::find()
            .filter(SchedulerEntity::Column::IsActive.eq(true))
            .filter(SchedulerEntity::Column::NextRunAt.lte(now))
            .all(&self.db)
            .await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match SchedulerEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(model) => { model.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }
}
