use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

use crate::workflow_template::entity::WorkflowTemplateEntity;
use crate::workflow_template::entity::WorkflowTemplateEntity::Model;

pub struct WorkflowTemplateSeaOrmRepository {
    db: DatabaseConnection,
}

impl WorkflowTemplateSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: WorkflowTemplateEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: WorkflowTemplateEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        WorkflowTemplateEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_all_by_key_and_tenant(&self, tenant_id: Uuid, key: &str) -> Result<Vec<Model>, sea_orm::DbErr> {
        WorkflowTemplateEntity::Entity::find()
            .filter(WorkflowTemplateEntity::Column::TenantId.eq(tenant_id))
            .filter(WorkflowTemplateEntity::Column::Key.eq(key))
            .all(&self.db)
            .await
    }

    pub async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        WorkflowTemplateEntity::Entity::find()
            .filter(WorkflowTemplateEntity::Column::TenantId.eq(tenant_id))
            .all(&self.db)
            .await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match WorkflowTemplateEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(model) => { model.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }
}
