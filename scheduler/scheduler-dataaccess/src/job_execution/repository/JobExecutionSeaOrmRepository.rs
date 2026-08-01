use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use crate::job_execution::entity::JobExecutionEntity;
use crate::job_execution::entity::JobExecutionEntity::Model;

pub struct JobExecutionSeaOrmRepository {
    db: DatabaseConnection,
}

impl JobExecutionSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: JobExecutionEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: JobExecutionEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        JobExecutionEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_scheduler_id(&self, scheduler_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        JobExecutionEntity::Entity::find()
            .filter(JobExecutionEntity::Column::SchedulerId.eq(scheduler_id))
            .order_by_desc(JobExecutionEntity::Column::StartedAt)
            .all(&self.db)
            .await
    }
}
