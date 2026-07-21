use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::AgentExecution::AgentExecution;
use sdlc_domain::port::output::AgentExecutionRepositoryPort::AgentExecutionRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::agent_execution::mapper::AgentExecutionDataMapper::AgentExecutionDataMapper;
use crate::agent_execution::repository::AgentExecutionSeaOrmRepository::AgentExecutionSeaOrmRepository;

pub struct AgentExecutionRepositoryImpl {
    sea_orm_repo: AgentExecutionSeaOrmRepository,
}

impl AgentExecutionRepositoryImpl {
    pub fn new(sea_orm_repo: AgentExecutionSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl AgentExecutionRepositoryPort for AgentExecutionRepositoryImpl {
    async fn save(&self, execution: &AgentExecution) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(AgentExecutionDataMapper::to_active_model(execution)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save agent execution"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, execution: &AgentExecution) -> Result<(), DomainError> {
        self.sea_orm_repo.update(AgentExecutionDataMapper::to_active_model(execution)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update agent execution"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<AgentExecution>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(AgentExecutionDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find agent execution"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_agent_id(&self, agent_id: Uuid) -> Result<Vec<AgentExecution>, DomainError> {
        self.sea_orm_repo.find_by_agent_id(agent_id).await
            .map(|models| models.iter().map(AgentExecutionDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list agent executions"); DomainError::InternalError(e.to_string()) })
    }
}
