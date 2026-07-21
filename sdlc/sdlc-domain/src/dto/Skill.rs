use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Skill {
    pub id: Uuid,
    /// `None` marks a global/platform-wide skill available to all tenants.
    pub tenant_id: Option<Uuid>,
    pub name: String,
    /// Tells an agent (or a human picking skills for one) when this skill applies.
    pub description: String,
    /// The instructions folded into an agent's system prompt when the skill is attached.
    pub content: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

pub struct NewSkill {
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub content: String,
}

impl Skill {
    pub fn new(fields: NewSkill) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: fields.tenant_id,
            name: fields.name,
            description: fields.description,
            content: fields.content,
            is_active: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}
