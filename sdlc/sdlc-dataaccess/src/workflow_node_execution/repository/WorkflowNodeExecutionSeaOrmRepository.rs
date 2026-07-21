use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use crate::workflow_node_execution::entity::WorkflowNodeExecutionEntity;
use crate::workflow_node_execution::entity::WorkflowNodeExecutionEntity::Model;

pub struct WorkflowNodeExecutionSeaOrmRepository {
    db: DatabaseConnection,
}

impl WorkflowNodeExecutionSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: WorkflowNodeExecutionEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: WorkflowNodeExecutionEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_execution_id(&self, execution_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        WorkflowNodeExecutionEntity::Entity::find()
            .filter(WorkflowNodeExecutionEntity::Column::WorkflowExecutionId.eq(execution_id))
            .all(&self.db)
            .await
    }

    pub async fn find_latest_by_execution_and_node(&self, execution_id: Uuid, node_id: &str) -> Result<Option<Model>, sea_orm::DbErr> {
        WorkflowNodeExecutionEntity::Entity::find()
            .filter(WorkflowNodeExecutionEntity::Column::WorkflowExecutionId.eq(execution_id))
            .filter(WorkflowNodeExecutionEntity::Column::NodeId.eq(node_id))
            .order_by_desc(WorkflowNodeExecutionEntity::Column::Attempt)
            .one(&self.db)
            .await
    }
}
