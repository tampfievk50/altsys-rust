use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::user::entity::UserEntity;
use crate::user::entity::UserRoleEntity;

pub struct UserSeaOrmRepository {
    db: DatabaseConnection,
}

impl UserSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: UserEntity::ActiveModel) -> Result<UserEntity::Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: UserEntity::ActiveModel) -> Result<UserEntity::Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<UserEntity::Model>, sea_orm::DbErr> {
        UserEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_username_and_tenant(&self, username: &str, tenant_id: Uuid) -> Result<Option<UserEntity::Model>, sea_orm::DbErr> {
        UserEntity::Entity::find()
            .filter(UserEntity::Column::Username.eq(username))
            .filter(UserEntity::Column::TenantId.eq(tenant_id))
            .one(&self.db)
            .await
    }

    pub async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<UserEntity::Model>, sea_orm::DbErr> {
        UserEntity::Entity::find()
            .filter(UserEntity::Column::TenantId.eq(tenant_id))
            .all(&self.db)
            .await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match UserEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(model) => { model.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }

    pub async fn insert_user_role(&self, user_id: Uuid, role_id: Uuid, tenant_id: Uuid) -> Result<(), sea_orm::DbErr> {
        let active = UserRoleEntity::ActiveModel {
            user_id: Set(user_id),
            role_id: Set(role_id),
            tenant_id: Set(tenant_id),
        };
        active.insert(&self.db).await?;
        Ok(())
    }

    pub async fn delete_user_role(&self, user_id: Uuid, role_id: Uuid) -> Result<bool, sea_orm::DbErr> {
        let model = UserRoleEntity::Entity::find()
            .filter(UserRoleEntity::Column::UserId.eq(user_id))
            .filter(UserRoleEntity::Column::RoleId.eq(role_id))
            .one(&self.db)
            .await?;
        match model {
            Some(m) => { m.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }

    pub async fn find_roles_by_user(&self, user_id: Uuid) -> Result<Vec<UserRoleEntity::Model>, sea_orm::DbErr> {
        UserRoleEntity::Entity::find()
            .filter(UserRoleEntity::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
    }
}
