use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

use crate::agent_skill::entity::AgentSkillEntity;
use crate::skill::entity::SkillEntity;
use crate::skill::entity::SkillEntity::Model;

pub struct SkillSeaOrmRepository {
    db: DatabaseConnection,
}

impl SkillSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, skill: SkillEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        skill.insert(&self.db).await
    }

    pub async fn update(&self, skill: SkillEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        skill.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        SkillEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        SkillEntity::Entity::find()
            .filter(
                Condition::any()
                    .add(SkillEntity::Column::TenantId.eq(tenant_id))
                    .add(SkillEntity::Column::TenantId.is_null()),
            )
            .all(&self.db)
            .await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match SkillEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(skill) => { skill.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }

    /// Full-replace: deletes every existing `agent_skills` row for `agent_id`
    /// and inserts one row per `skill_ids`. Not wrapped in a DB transaction —
    /// matches the rest of this codebase's repositories, none of which use
    /// multi-statement transactions either.
    pub async fn set_skills_for_agent(&self, agent_id: Uuid, skill_ids: &[Uuid]) -> Result<(), sea_orm::DbErr> {
        AgentSkillEntity::Entity::delete_many()
            .filter(AgentSkillEntity::Column::AgentId.eq(agent_id))
            .exec(&self.db)
            .await?;

        if skill_ids.is_empty() {
            return Ok(());
        }

        let rows = skill_ids.iter().map(|skill_id| AgentSkillEntity::ActiveModel {
            agent_id: sea_orm::Set(agent_id),
            skill_id: sea_orm::Set(*skill_id),
        });
        AgentSkillEntity::Entity::insert_many(rows).exec(&self.db).await?;
        Ok(())
    }

    pub async fn find_skill_ids_by_agent(&self, agent_id: Uuid) -> Result<Vec<Uuid>, sea_orm::DbErr> {
        let rows = AgentSkillEntity::Entity::find()
            .filter(AgentSkillEntity::Column::AgentId.eq(agent_id))
            .all(&self.db)
            .await?;
        Ok(rows.into_iter().map(|r| r.skill_id).collect())
    }

    pub async fn find_active_skills_by_agent(&self, agent_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        let skill_ids = self.find_skill_ids_by_agent(agent_id).await?;
        if skill_ids.is_empty() {
            return Ok(vec![]);
        }
        SkillEntity::Entity::find()
            .filter(SkillEntity::Column::Id.is_in(skill_ids))
            .filter(SkillEntity::Column::IsActive.eq(true))
            .all(&self.db)
            .await
    }
}
