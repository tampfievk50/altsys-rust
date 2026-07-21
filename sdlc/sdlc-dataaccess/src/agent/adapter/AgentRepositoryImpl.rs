use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::Agent::Agent;
use sdlc_domain::port::output::AgentRepositoryPort::AgentRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::agent::mapper::AgentDataMapper::AgentDataMapper;
use crate::agent::repository::AgentSeaOrmRepository::AgentSeaOrmRepository;

pub struct AgentRepositoryImpl {
    sea_orm_repo: AgentSeaOrmRepository,
}

impl AgentRepositoryImpl {
    pub fn new(sea_orm_repo: AgentSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl AgentRepositoryPort for AgentRepositoryImpl {
    async fn save(&self, agent: &Agent) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(AgentDataMapper::to_active_model(agent)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save agent"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, agent: &Agent) -> Result<(), DomainError> {
        self.sea_orm_repo.update(AgentDataMapper::to_active_model(agent)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update agent"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Agent>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(AgentDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find agent"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Agent>, DomainError> {
        self.sea_orm_repo.find_by_tenant_including_global(tenant_id).await
            .map(|agents| agents.iter().map(AgentDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list agents"); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(error = %e, "Failed to delete agent"); DomainError::InternalError(e.to_string()) })
    }
}
