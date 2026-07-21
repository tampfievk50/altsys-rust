use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::IngestedEvent::IngestedEvent;
use sdlc_domain::port::output::EventRepositoryPort::EventRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::event::mapper::EventDataMapper::EventDataMapper;
use crate::event::repository::EventSeaOrmRepository::EventSeaOrmRepository;

pub struct EventRepositoryImpl {
    sea_orm_repo: EventSeaOrmRepository,
}

impl EventRepositoryImpl {
    pub fn new(sea_orm_repo: EventSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl EventRepositoryPort for EventRepositoryImpl {
    async fn save(&self, event: &IngestedEvent) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(EventDataMapper::to_active_model(event)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save event"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<IngestedEvent>, DomainError> {
        self.sea_orm_repo.find_by_tenant(tenant_id).await
            .map(|models| models.iter().map(EventDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list events"); DomainError::InternalError(e.to_string()) })
    }
}
