use sea_orm::Set;

use sdlc_domain::dto::AutomationRule::AutomationRule;

use crate::automation_rule::entity::AutomationRuleEntity;

pub struct AutomationRuleDataMapper;

impl AutomationRuleDataMapper {
    pub fn to_domain(entity: &AutomationRuleEntity::Model) -> AutomationRule {
        AutomationRule {
            id: entity.id,
            tenant_id: entity.tenant_id,
            name: entity.name.clone(),
            event_type: entity.event_type.clone(),
            match_criteria: entity.match_criteria.clone(),
            action: entity.action.clone(),
            is_active: entity.is_active,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            created_by: entity.created_by,
            updated_by: entity.updated_by,
        }
    }

    pub fn to_active_model(rule: &AutomationRule) -> AutomationRuleEntity::ActiveModel {
        AutomationRuleEntity::ActiveModel {
            id: Set(rule.id),
            tenant_id: Set(rule.tenant_id),
            name: Set(rule.name.clone()),
            event_type: Set(rule.event_type.clone()),
            match_criteria: Set(rule.match_criteria.clone()),
            action: Set(rule.action.clone()),
            is_active: Set(rule.is_active),
            created_at: Set(rule.created_at),
            updated_at: Set(rule.updated_at),
            created_by: Set(rule.created_by),
            updated_by: Set(rule.updated_by),
        }
    }
}
