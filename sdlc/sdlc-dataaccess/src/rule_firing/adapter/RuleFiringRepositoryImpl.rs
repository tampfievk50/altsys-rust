use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::RuleFiring::RuleFiring;
use sdlc_domain::port::output::RuleFiringRepositoryPort::RuleFiringRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::rule_firing::mapper::RuleFiringDataMapper::RuleFiringDataMapper;
use crate::rule_firing::repository::RuleFiringSeaOrmRepository::RuleFiringSeaOrmRepository;

pub struct RuleFiringRepositoryImpl {
    sea_orm_repo: RuleFiringSeaOrmRepository,
}

impl RuleFiringRepositoryImpl {
    pub fn new(sea_orm_repo: RuleFiringSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl RuleFiringRepositoryPort for RuleFiringRepositoryImpl {
    async fn save(&self, firing: &RuleFiring) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(RuleFiringDataMapper::to_active_model(firing)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save rule firing"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_event_id(&self, event_id: Uuid) -> Result<Vec<RuleFiring>, DomainError> {
        self.sea_orm_repo.find_by_event_id(event_id).await
            .map(|models| models.iter().map(RuleFiringDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list rule firings"); DomainError::InternalError(e.to_string()) })
    }
}
