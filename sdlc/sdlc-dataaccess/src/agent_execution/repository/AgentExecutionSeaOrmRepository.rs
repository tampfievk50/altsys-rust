use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::agent_execution::entity::AgentExecutionEntity;
use crate::agent_execution::entity::AgentExecutionEntity::Model;

pub struct AgentExecutionSeaOrmRepository {
    db: DatabaseConnection,
}

impl AgentExecutionSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: AgentExecutionEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: AgentExecutionEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        AgentExecutionEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_agent_id(&self, agent_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        AgentExecutionEntity::Entity::find()
            .filter(AgentExecutionEntity::Column::AgentId.eq(agent_id))
            .all(&self.db)
            .await
    }
}
