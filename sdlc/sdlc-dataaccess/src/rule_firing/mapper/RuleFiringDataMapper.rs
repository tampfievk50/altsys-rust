use std::str::FromStr;

use sea_orm::Set;

use sdlc_domain::dto::RuleFiring::RuleFiring;
use sdlc_domain::dto::RuleFiringStatus::RuleFiringStatus;

use crate::rule_firing::entity::RuleFiringEntity;

pub struct RuleFiringDataMapper;

impl RuleFiringDataMapper {
    pub fn to_domain(entity: &RuleFiringEntity::Model) -> RuleFiring {
        RuleFiring {
            id: entity.id,
            event_id: entity.event_id,
            rule_id: entity.rule_id,
            matched: entity.matched,
            status: RuleFiringStatus::from_str(&entity.status).unwrap_or(RuleFiringStatus::Failed),
            action_result: entity.action_result.as_deref().and_then(|s| serde_json::from_str(s).ok()),
            error: entity.error.clone(),
            created_at: entity.created_at,
        }
    }

    pub fn to_active_model(firing: &RuleFiring) -> RuleFiringEntity::ActiveModel {
        RuleFiringEntity::ActiveModel {
            id: Set(firing.id),
            event_id: Set(firing.event_id),
            rule_id: Set(firing.rule_id),
            matched: Set(firing.matched),
            status: Set(firing.status.to_string()),
            action_result: Set(firing.action_result.as_ref().map(|v| v.to_string())),
            error: Set(firing.error.clone()),
            created_at: Set(firing.created_at),
        }
    }
}
