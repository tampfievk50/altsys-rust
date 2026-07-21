use sea_orm::entity::prelude::*;

/// The `agent_skills` join table. No domain-layer counterpart of its own —
/// it only ever exists as query plumbing behind `SkillRepositoryPort`'s
/// agent-skill methods.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "agent_skills")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub agent_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub skill_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
