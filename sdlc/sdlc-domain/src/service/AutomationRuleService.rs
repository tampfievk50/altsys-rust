use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::ActionSpec::ActionSpec;
use crate::dto::AutomationRule::{AutomationRule, NewAutomationRule};
use crate::dto::AutomationRuleResponse::AutomationRuleResponse;
use crate::dto::CreateAutomationRuleCommand::CreateAutomationRuleCommand;
use crate::dto::UpdateAutomationRuleCommand::UpdateAutomationRuleCommand;
use crate::port::input::AutomationRulePort::AutomationRulePort;
use crate::port::output::AutomationRuleRepositoryPort::AutomationRuleRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct AutomationRuleService {
    rule_repository: Arc<dyn AutomationRuleRepositoryPort>,
}

impl AutomationRuleService {
    pub fn new(rule_repository: Arc<dyn AutomationRuleRepositoryPort>) -> Self {
        Self { rule_repository }
    }

    fn to_response(rule: &AutomationRule) -> AutomationRuleResponse {
        AutomationRuleResponse {
            id: rule.id,
            tenant_id: rule.tenant_id,
            name: rule.name.clone(),
            event_type: rule.event_type.clone(),
            match_criteria: rule.match_criteria.clone(),
            action: rule.action.clone(),
            is_active: rule.is_active,
            created_at: rule.created_at,
            updated_at: rule.updated_at,
            created_by: rule.created_by,
            updated_by: rule.updated_by,
        }
    }
}

#[async_trait]
impl AutomationRulePort for AutomationRuleService {
    async fn create_rule(&self, command: CreateAutomationRuleCommand) -> Result<AutomationRuleResponse, DomainError> {
        info!(name = %command.name, event_type = %command.event_type, "Creating automation rule");
        if command.name.trim().is_empty() {
            return Err(DomainError::ValidationError("Name cannot be empty".into()));
        }
        if command.event_type.trim().is_empty() {
            return Err(DomainError::ValidationError("Event type cannot be empty".into()));
        }
        // Validated eagerly so a malformed action is rejected at creation time,
        // not the first time a matching event tries to dispatch it.
        ActionSpec::parse(&command.action)?;

        let rule = AutomationRule::new(NewAutomationRule {
            tenant_id: command.tenant_id,
            name: command.name,
            event_type: command.event_type,
            match_criteria: command.match_criteria,
            action: command.action,
        });
        self.rule_repository.save(&rule).await?;
        info!(rule_id = %rule.id, "Automation rule created");
        Ok(Self::to_response(&rule))
    }

    async fn find_rule_by_id(&self, id: Uuid) -> Result<AutomationRuleResponse, DomainError> {
        let rule = self.rule_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Automation rule not found: {}", id)))?;
        Ok(Self::to_response(&rule))
    }

    async fn find_rules_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<AutomationRuleResponse>, DomainError> {
        let rules = self.rule_repository.find_by_tenant(tenant_id).await?;
        Ok(rules.iter().map(Self::to_response).collect())
    }

    async fn update_rule(&self, id: Uuid, command: UpdateAutomationRuleCommand) -> Result<AutomationRuleResponse, DomainError> {
        info!(rule_id = %id, "Updating automation rule");
        let mut rule = self.rule_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Automation rule not found: {}", id)))?;
        if let Some(name) = command.name {
            rule.name = name;
        }
        if let Some(match_criteria) = command.match_criteria {
            rule.match_criteria = Some(match_criteria);
        }
        if let Some(action) = command.action {
            ActionSpec::parse(&action)?;
            rule.action = action;
        }
        if let Some(is_active) = command.is_active {
            rule.is_active = is_active;
        }
        rule.updated_at = Utc::now();
        self.rule_repository.update(&rule).await?;
        Ok(Self::to_response(&rule))
    }

    async fn delete_rule(&self, id: Uuid) -> Result<(), DomainError> {
        info!(rule_id = %id, "Deleting automation rule");
        let deleted = self.rule_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(rule_id = %id, "Automation rule not found for deletion");
            return Err(DomainError::NotFound(format!("Automation rule not found: {}", id)));
        }
        Ok(())
    }
}
