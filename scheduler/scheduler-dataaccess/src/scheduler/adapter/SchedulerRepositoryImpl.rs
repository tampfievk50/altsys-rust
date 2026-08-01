use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tracing::error;
use uuid::Uuid;

use scheduler_domain::dto::Scheduler::Scheduler;
use scheduler_domain::port::output::SchedulerRepositoryPort::SchedulerRepositoryPort;
use scheduler_domain::r#enum::DomainError::DomainError;

use crate::scheduler::mapper::SchedulerDataMapper::SchedulerDataMapper;
use crate::scheduler::repository::SchedulerSeaOrmRepository::SchedulerSeaOrmRepository;

pub struct SchedulerRepositoryImpl {
    sea_orm_repo: SchedulerSeaOrmRepository,
}

impl SchedulerRepositoryImpl {
    pub fn new(sea_orm_repo: SchedulerSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl SchedulerRepositoryPort for SchedulerRepositoryImpl {
    async fn save(&self, scheduler: &Scheduler) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(SchedulerDataMapper::to_active_model(scheduler)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save scheduler"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, scheduler: &Scheduler) -> Result<(), DomainError> {
        self.sea_orm_repo.update(SchedulerDataMapper::to_active_model(scheduler)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update scheduler"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Scheduler>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(SchedulerDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find scheduler"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_all(&self) -> Result<Vec<Scheduler>, DomainError> {
        self.sea_orm_repo.find_all().await
            .map(|models| models.iter().map(SchedulerDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list schedulers"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_due(&self, now: DateTime<Utc>) -> Result<Vec<Scheduler>, DomainError> {
        self.sea_orm_repo.find_due(now).await
            .map(|models| models.iter().map(SchedulerDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list due schedulers"); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(error = %e, "Failed to delete scheduler"); DomainError::InternalError(e.to_string()) })
    }
}
