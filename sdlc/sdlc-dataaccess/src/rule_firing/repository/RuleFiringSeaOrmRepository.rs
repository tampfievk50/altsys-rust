use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::rule_firing::entity::RuleFiringEntity;
use crate::rule_firing::entity::RuleFiringEntity::Model;

pub struct RuleFiringSeaOrmRepository {
    db: DatabaseConnection,
}

impl RuleFiringSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: RuleFiringEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn find_by_event_id(&self, event_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        RuleFiringEntity::Entity::find()
            .filter(RuleFiringEntity::Column::EventId.eq(event_id))
            .all(&self.db)
            .await
    }
}
