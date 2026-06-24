use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait};
use uuid::Uuid;

use crate::user::entity::UserEntity;
use crate::user::entity::UserEntity::Model;

pub struct UserSeaOrmRepository {
    db: DatabaseConnection,
}

impl UserSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, active_model: UserEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        active_model.insert(&self.db).await
    }

    pub async fn update(&self, active_model: UserEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        active_model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        UserEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_all(&self) -> Result<Vec<Model>, sea_orm::DbErr> {
        UserEntity::Entity::find().all(&self.db).await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        let result = UserEntity::Entity::find_by_id(id).one(&self.db).await?;
        match result {
            Some(model) => {
                model.delete(&self.db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
