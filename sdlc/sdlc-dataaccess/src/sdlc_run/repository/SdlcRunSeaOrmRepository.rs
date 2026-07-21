use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::sdlc_run::entity::SdlcRunEntity;
use crate::sdlc_run::entity::SdlcRunEntity::Model;

pub struct SdlcRunSeaOrmRepository {
    db: DatabaseConnection,
}

impl SdlcRunSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: SdlcRunEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: SdlcRunEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        SdlcRunEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        SdlcRunEntity::Entity::find()
            .filter(SdlcRunEntity::Column::TenantId.eq(tenant_id))
            .all(&self.db)
            .await
    }
}
