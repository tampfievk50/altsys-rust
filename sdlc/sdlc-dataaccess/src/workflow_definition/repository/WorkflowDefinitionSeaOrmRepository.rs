use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

use crate::workflow_definition::entity::WorkflowDefinitionEntity;
use crate::workflow_definition::entity::WorkflowDefinitionEntity::Model;

pub struct WorkflowDefinitionSeaOrmRepository {
    db: DatabaseConnection,
}

impl WorkflowDefinitionSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: WorkflowDefinitionEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: WorkflowDefinitionEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        WorkflowDefinitionEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_all_by_key_and_tenant(&self, tenant_id: Uuid, key: &str) -> Result<Vec<Model>, sea_orm::DbErr> {
        WorkflowDefinitionEntity::Entity::find()
            .filter(WorkflowDefinitionEntity::Column::TenantId.eq(tenant_id))
            .filter(WorkflowDefinitionEntity::Column::Key.eq(key))
            .all(&self.db)
            .await
    }

    pub async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        WorkflowDefinitionEntity::Entity::find()
            .filter(WorkflowDefinitionEntity::Column::TenantId.eq(tenant_id))
            .all(&self.db)
            .await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match WorkflowDefinitionEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(model) => { model.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }
}
