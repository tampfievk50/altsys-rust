use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::AutomationRuleResponse::AutomationRuleResponse;
use crate::dto::CreateAutomationRuleCommand::CreateAutomationRuleCommand;
use crate::dto::UpdateAutomationRuleCommand::UpdateAutomationRuleCommand;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait AutomationRulePort: Send + Sync {
    async fn create_rule(&self, command: CreateAutomationRuleCommand) -> Result<AutomationRuleResponse, DomainError>;
    async fn find_rule_by_id(&self, id: Uuid) -> Result<AutomationRuleResponse, DomainError>;
    async fn find_rules_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<AutomationRuleResponse>, DomainError>;
    async fn update_rule(&self, id: Uuid, command: UpdateAutomationRuleCommand) -> Result<AutomationRuleResponse, DomainError>;
    async fn delete_rule(&self, id: Uuid) -> Result<(), DomainError>;
}
