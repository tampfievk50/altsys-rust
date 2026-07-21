use sea_orm::Set;

use sdlc_domain::dto::Skill::Skill;

use crate::skill::entity::SkillEntity;

pub struct SkillDataMapper;

impl SkillDataMapper {
    pub fn to_domain(entity: &SkillEntity::Model) -> Skill {
        Skill {
            id: entity.id,
            tenant_id: entity.tenant_id,
            name: entity.name.clone(),
            description: entity.description.clone(),
            content: entity.content.clone(),
            is_active: entity.is_active,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            created_by: entity.created_by,
            updated_by: entity.updated_by,
        }
    }

    pub fn to_active_model(skill: &Skill) -> SkillEntity::ActiveModel {
        SkillEntity::ActiveModel {
            id: Set(skill.id),
            tenant_id: Set(skill.tenant_id),
            name: Set(skill.name.clone()),
            description: Set(skill.description.clone()),
            content: Set(skill.content.clone()),
            is_active: Set(skill.is_active),
            created_at: Set(skill.created_at),
            updated_at: Set(skill.updated_at),
            created_by: Set(skill.created_by),
            updated_by: Set(skill.updated_by),
        }
    }
}
