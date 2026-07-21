use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

use crate::automation_rule::entity::AutomationRuleEntity;
use crate::automation_rule::entity::AutomationRuleEntity::Model;

pub struct AutomationRuleSeaOrmRepository {
    db: DatabaseConnection,
}

impl AutomationRuleSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, rule: AutomationRuleEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        rule.insert(&self.db).await
    }

    pub async fn update(&self, rule: AutomationRuleEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        rule.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        AutomationRuleEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        AutomationRuleEntity::Entity::find()
            .filter(AutomationRuleEntity::Column::TenantId.eq(tenant_id))
            .all(&self.db)
            .await
    }

    pub async fn find_active_by_tenant_and_event_type(&self, tenant_id: Uuid, event_type: &str) -> Result<Vec<Model>, sea_orm::DbErr> {
        AutomationRuleEntity::Entity::find()
            .filter(AutomationRuleEntity::Column::TenantId.eq(tenant_id))
            .filter(AutomationRuleEntity::Column::EventType.eq(event_type))
            .filter(AutomationRuleEntity::Column::IsActive.eq(true))
            .all(&self.db)
            .await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match AutomationRuleEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(rule) => { rule.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }
}
