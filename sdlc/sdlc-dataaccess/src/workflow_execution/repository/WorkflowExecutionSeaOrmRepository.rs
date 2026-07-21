use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::workflow_execution::entity::WorkflowExecutionEntity;
use crate::workflow_execution::entity::WorkflowExecutionEntity::Model;

pub struct WorkflowExecutionSeaOrmRepository {
    db: DatabaseConnection,
}

impl WorkflowExecutionSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: WorkflowExecutionEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: WorkflowExecutionEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        WorkflowExecutionEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        WorkflowExecutionEntity::Entity::find()
            .filter(WorkflowExecutionEntity::Column::TenantId.eq(tenant_id))
            .all(&self.db)
            .await
    }
}
