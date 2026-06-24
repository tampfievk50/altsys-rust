use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::download::entity::DownloadEntity;
use crate::download::entity::DownloadEntity::Model;

pub struct DownloadSeaOrmRepository {
    db: DatabaseConnection,
}

impl DownloadSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, active_model: DownloadEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        active_model.insert(&self.db).await
    }

    pub async fn update(&self, active_model: DownloadEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        active_model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        DownloadEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_all(&self) -> Result<Vec<Model>, sea_orm::DbErr> {
        DownloadEntity::Entity::find().all(&self.db).await
    }
}
