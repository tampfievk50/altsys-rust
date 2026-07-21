use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

use crate::agent::entity::AgentEntity;
use crate::agent::entity::AgentEntity::Model;

pub struct AgentSeaOrmRepository {
    db: DatabaseConnection,
}

impl AgentSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, agent: AgentEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        agent.insert(&self.db).await
    }

    pub async fn update(&self, agent: AgentEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        agent.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        AgentEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        AgentEntity::Entity::find()
            .filter(
                Condition::any()
                    .add(AgentEntity::Column::TenantId.eq(tenant_id))
                    .add(AgentEntity::Column::TenantId.is_null()),
            )
            .all(&self.db)
            .await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match AgentEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(agent) => { agent.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }
}
