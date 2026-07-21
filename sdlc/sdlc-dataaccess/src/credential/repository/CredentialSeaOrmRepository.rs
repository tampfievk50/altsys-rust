use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

use crate::credential::entity::CredentialEntity;
use crate::credential::entity::CredentialEntity::Model;

pub struct CredentialSeaOrmRepository {
    db: DatabaseConnection,
}

impl CredentialSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: CredentialEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: CredentialEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        CredentialEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        CredentialEntity::Entity::find()
            .filter(CredentialEntity::Column::TenantId.eq(tenant_id))
            .all(&self.db)
            .await
    }

    pub async fn find_by_name_and_tenant(&self, name: &str, tenant_id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        CredentialEntity::Entity::find()
            .filter(CredentialEntity::Column::Name.eq(name))
            .filter(CredentialEntity::Column::TenantId.eq(tenant_id))
            .one(&self.db)
            .await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match CredentialEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(model) => { model.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }
}
