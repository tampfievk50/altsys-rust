use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::task_override::entity::TaskOverrideEntity;
use crate::task_override::entity::TaskOverrideEntity::Model;

pub struct TaskOverrideSeaOrmRepository {
    db: DatabaseConnection,
}

impl TaskOverrideSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: TaskOverrideEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: TaskOverrideEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_project(&self, project_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        TaskOverrideEntity::Entity::find()
            .filter(TaskOverrideEntity::Column::ProjectId.eq(project_id))
            .all(&self.db)
            .await
    }

    pub async fn find_by_project_and_ticket(&self, project_id: Uuid, ticket_key: &str) -> Result<Option<Model>, sea_orm::DbErr> {
        TaskOverrideEntity::Entity::find()
            .filter(TaskOverrideEntity::Column::ProjectId.eq(project_id))
            .filter(TaskOverrideEntity::Column::TicketKey.eq(ticket_key))
            .one(&self.db)
            .await
    }
}
