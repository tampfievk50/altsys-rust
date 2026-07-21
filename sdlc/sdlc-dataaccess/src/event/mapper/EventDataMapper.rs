use sea_orm::Set;

use sdlc_domain::dto::IngestedEvent::IngestedEvent;

use crate::event::entity::EventEntity;

pub struct EventDataMapper;

impl EventDataMapper {
    pub fn to_domain(entity: &EventEntity::Model) -> IngestedEvent {
        IngestedEvent {
            id: entity.id,
            tenant_id: entity.tenant_id,
            event_type: entity.event_type.clone(),
            payload: serde_json::from_str(&entity.payload).unwrap_or_else(|_| serde_json::json!({})),
            received_at: entity.received_at,
        }
    }

    pub fn to_active_model(event: &IngestedEvent) -> EventEntity::ActiveModel {
        EventEntity::ActiveModel {
            id: Set(event.id),
            tenant_id: Set(event.tenant_id),
            event_type: Set(event.event_type.clone()),
            payload: Set(event.payload.to_string()),
            received_at: Set(event.received_at),
        }
    }
}
