use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

// No `Eq` derive: `temperature: Option<f32>` only implements `PartialEq`.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "agents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub agent_type: String,
    pub system_prompt: String,
    pub provider: String,
    pub model: String,
    pub temperature: Option<f32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
