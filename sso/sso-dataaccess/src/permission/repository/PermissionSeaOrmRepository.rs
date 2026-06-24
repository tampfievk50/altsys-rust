use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;
use crate::permission::entity::PermissionEntity;

pub struct PermissionSeaOrmRepository {
    db: DatabaseConnection,
}

impl PermissionSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: PermissionEntity::ActiveModel) -> Result<PermissionEntity::Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: PermissionEntity::ActiveModel) -> Result<PermissionEntity::Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<PermissionEntity::Model>, sea_orm::DbErr> {
        PermissionEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Option<PermissionEntity::Model>, sea_orm::DbErr> {
        PermissionEntity::Entity::find()
            .filter(PermissionEntity::Column::Name.eq(name))
            .one(&self.db)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<PermissionEntity::Model>, sea_orm::DbErr> {
        PermissionEntity::Entity::find().all(&self.db).await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match PermissionEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(model) => { model.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }
}
