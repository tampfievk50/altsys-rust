use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::event::entity::EventEntity;
use crate::event::entity::EventEntity::Model;

pub struct EventSeaOrmRepository {
    db: DatabaseConnection,
}

impl EventSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: EventEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        EventEntity::Entity::find()
            .filter(EventEntity::Column::TenantId.eq(tenant_id))
            .all(&self.db)
            .await
    }
}
