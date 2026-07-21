use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::AutomationRule::AutomationRule;
use sdlc_domain::port::output::AutomationRuleRepositoryPort::AutomationRuleRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::automation_rule::mapper::AutomationRuleDataMapper::AutomationRuleDataMapper;
use crate::automation_rule::repository::AutomationRuleSeaOrmRepository::AutomationRuleSeaOrmRepository;

pub struct AutomationRuleRepositoryImpl {
    sea_orm_repo: AutomationRuleSeaOrmRepository,
}

impl AutomationRuleRepositoryImpl {
    pub fn new(sea_orm_repo: AutomationRuleSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl AutomationRuleRepositoryPort for AutomationRuleRepositoryImpl {
    async fn save(&self, rule: &AutomationRule) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(AutomationRuleDataMapper::to_active_model(rule)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save automation rule"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, rule: &AutomationRule) -> Result<(), DomainError> {
        self.sea_orm_repo.update(AutomationRuleDataMapper::to_active_model(rule)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update automation rule"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<AutomationRule>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(AutomationRuleDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find automation rule"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<AutomationRule>, DomainError> {
        self.sea_orm_repo.find_by_tenant(tenant_id).await
            .map(|rules| rules.iter().map(AutomationRuleDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list automation rules"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_active_by_tenant_and_event_type(&self, tenant_id: Uuid, event_type: &str) -> Result<Vec<AutomationRule>, DomainError> {
        self.sea_orm_repo.find_active_by_tenant_and_event_type(tenant_id, event_type).await
            .map(|rules| rules.iter().map(AutomationRuleDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list active automation rules"); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(error = %e, "Failed to delete automation rule"); DomainError::InternalError(e.to_string()) })
    }
}
