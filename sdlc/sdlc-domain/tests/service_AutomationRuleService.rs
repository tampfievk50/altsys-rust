use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;
use sdlc_domain::dto::ActionSpec::ActionSpec;
use sdlc_domain::dto::AutomationRule::{AutomationRule, NewAutomationRule};
use sdlc_domain::dto::AutomationRuleResponse::AutomationRuleResponse;
use sdlc_domain::dto::CreateAutomationRuleCommand::CreateAutomationRuleCommand;
use sdlc_domain::dto::UpdateAutomationRuleCommand::UpdateAutomationRuleCommand;
use sdlc_domain::port::input::AutomationRulePort::AutomationRulePort;
use sdlc_domain::port::output::AutomationRuleRepositoryPort::AutomationRuleRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::AutomationRuleService::AutomationRuleService;

use std::sync::Mutex;

#[derive(Default)]
struct MockRuleRepository {
    rules: Mutex<Vec<AutomationRule>>,
}

#[async_trait]
impl AutomationRuleRepositoryPort for MockRuleRepository {
    async fn save(&self, rule: &AutomationRule) -> Result<(), DomainError> {
        self.rules.lock().unwrap().push(rule.clone());
        Ok(())
    }
    async fn update(&self, rule: &AutomationRule) -> Result<(), DomainError> {
        let mut rules = self.rules.lock().unwrap();
        if let Some(existing) = rules.iter_mut().find(|r| r.id == rule.id) {
            *existing = rule.clone();
        }
        Ok(())
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AutomationRule>, DomainError> {
        Ok(self.rules.lock().unwrap().iter().find(|r| r.id == id).cloned())
    }
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<AutomationRule>, DomainError> {
        Ok(self.rules.lock().unwrap().iter().filter(|r| r.tenant_id == tenant_id).cloned().collect())
    }
    async fn find_active_by_tenant_and_event_type(&self, tenant_id: Uuid, event_type: &str) -> Result<Vec<AutomationRule>, DomainError> {
        Ok(self.rules.lock().unwrap().iter()
            .filter(|r| r.tenant_id == tenant_id && r.event_type == event_type && r.is_active)
            .cloned()
            .collect())
    }
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        let mut rules = self.rules.lock().unwrap();
        let len_before = rules.len();
        rules.retain(|r| r.id != id);
        Ok(rules.len() != len_before)
    }
}

fn sample_command(tenant_id: Uuid) -> CreateAutomationRuleCommand {
    CreateAutomationRuleCommand {
        tenant_id,
        name: "Auto-start on Ready for Dev".into(),
        event_type: "jira.ticket.transitioned".into(),
        match_criteria: Some(r#"{"status":"Ready for Dev"}"#.into()),
        action: r#"{"action_type":"start_sdlc_run","parameters":{}}"#.into(),
    }
}

#[tokio::test]
async fn create_rule_rejects_invalid_action_json() {
    let service = AutomationRuleService::new(Arc::new(MockRuleRepository::default()));
    let mut command = sample_command(Uuid::new_v4());
    command.action = "not json".into();
    let result = service.create_rule(command).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn create_rule_rejects_unknown_action_type() {
    let service = AutomationRuleService::new(Arc::new(MockRuleRepository::default()));
    let mut command = sample_command(Uuid::new_v4());
    command.action = r#"{"action_type":"launch_missiles"}"#.into();
    let result = service.create_rule(command).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn find_rules_by_tenant_only_returns_that_tenant() {
    let service = AutomationRuleService::new(Arc::new(MockRuleRepository::default()));
    let tenant_id = Uuid::new_v4();
    service.create_rule(sample_command(tenant_id)).await.unwrap();
    service.create_rule(sample_command(Uuid::new_v4())).await.unwrap();

    let results = service.find_rules_by_tenant(tenant_id).await.unwrap();
    assert_eq!(results.len(), 1);
}

#[tokio::test]
async fn delete_rule_fails_when_not_found() {
    let service = AutomationRuleService::new(Arc::new(MockRuleRepository::default()));
    let result = service.delete_rule(Uuid::new_v4()).await;
    assert!(matches!(result, Err(DomainError::NotFound(_))));
}
