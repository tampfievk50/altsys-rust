use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::role::entity::RoleEntity;
use crate::role::entity::RolePermissionEntity;

pub struct RoleSeaOrmRepository {
    db: DatabaseConnection,
}

impl RoleSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: RoleEntity::ActiveModel) -> Result<RoleEntity::Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: RoleEntity::ActiveModel) -> Result<RoleEntity::Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<RoleEntity::Model>, sea_orm::DbErr> {
        RoleEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<RoleEntity::Model>, sea_orm::DbErr> {
        RoleEntity::Entity::find()
            .filter(RoleEntity::Column::TenantId.eq(tenant_id))
            .all(&self.db)
            .await
    }

    pub async fn find_by_name_and_tenant(&self, name: &str, tenant_id: Uuid) -> Result<Option<RoleEntity::Model>, sea_orm::DbErr> {
        RoleEntity::Entity::find()
            .filter(RoleEntity::Column::Name.eq(name))
            .filter(RoleEntity::Column::TenantId.eq(tenant_id))
            .one(&self.db)
            .await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match RoleEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(model) => { model.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }

    pub async fn insert_role_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), sea_orm::DbErr> {
        let active = RolePermissionEntity::ActiveModel {
            role_id: Set(role_id),
            permission_id: Set(permission_id),
        };
        active.insert(&self.db).await?;
        Ok(())
    }

    pub async fn delete_role_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<bool, sea_orm::DbErr> {
        let model = RolePermissionEntity::Entity::find()
            .filter(RolePermissionEntity::Column::RoleId.eq(role_id))
            .filter(RolePermissionEntity::Column::PermissionId.eq(permission_id))
            .one(&self.db)
            .await?;
        match model {
            Some(m) => { m.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }

    pub async fn find_permissions_by_role(&self, role_id: Uuid) -> Result<Vec<RolePermissionEntity::Model>, sea_orm::DbErr> {
        RolePermissionEntity::Entity::find()
            .filter(RolePermissionEntity::Column::RoleId.eq(role_id))
            .all(&self.db)
            .await
    }
}
