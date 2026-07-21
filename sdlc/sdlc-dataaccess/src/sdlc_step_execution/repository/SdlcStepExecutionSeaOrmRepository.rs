use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::sdlc_step_execution::entity::SdlcStepExecutionEntity;
use crate::sdlc_step_execution::entity::SdlcStepExecutionEntity::Model;

pub struct SdlcStepExecutionSeaOrmRepository {
    db: DatabaseConnection,
}

impl SdlcStepExecutionSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: SdlcStepExecutionEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: SdlcStepExecutionEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_run_id(&self, run_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        SdlcStepExecutionEntity::Entity::find()
            .filter(SdlcStepExecutionEntity::Column::RunId.eq(run_id))
            .all(&self.db)
            .await
    }
}
