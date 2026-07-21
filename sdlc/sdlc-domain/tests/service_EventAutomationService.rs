use std::sync::Arc;
use async_trait::async_trait;
use tracing::{error, info};
use uuid::Uuid;
use sdlc_domain::dto::ActionSpec::ActionSpec;
use sdlc_domain::dto::EventResponse::EventResponse;
use sdlc_domain::dto::IngestEventCommand::IngestEventCommand;
use sdlc_domain::dto::IngestEventResponse::IngestEventResponse;
use sdlc_domain::dto::IngestedEvent::{IngestedEvent, NewIngestedEvent};
use sdlc_domain::dto::RuleFiring::{NewRuleFiring, RuleFiring};
use sdlc_domain::dto::RuleFiringResponse::RuleFiringResponse;
use sdlc_domain::dto::RuleFiringStatus::RuleFiringStatus;
use sdlc_domain::port::input::EventPort::EventPort;
use sdlc_domain::port::output::AutomationRuleRepositoryPort::AutomationRuleRepositoryPort;
use sdlc_domain::port::output::EventRepositoryPort::EventRepositoryPort;
use sdlc_domain::port::output::PluginDispatchPort::PluginDispatchPort;
use sdlc_domain::port::output::PluginRepositoryPort::PluginRepositoryPort;
use sdlc_domain::port::output::RuleFiringRepositoryPort::RuleFiringRepositoryPort;
use sdlc_domain::port::output::AgentsClientPort::AgentsClientPort;
use sdlc_domain::port::output::SdlcClientPort::SdlcClientPort;
use sdlc_domain::port::output::AutomationToolsClientPort::AutomationToolsClientPort;
use sdlc_domain::port::output::WorkflowClientPort::WorkflowClientPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::EventAutomationService::EventAutomationService;

use sdlc_domain::dto::AutomationRule::{AutomationRule, NewAutomationRule};
use sdlc_domain::dto::Plugin::Plugin;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

#[derive(Default)]
struct MockEventRepository {
    events: Mutex<Vec<IngestedEvent>>,
}
#[async_trait]
impl EventRepositoryPort for MockEventRepository {
    async fn save(&self, event: &IngestedEvent) -> Result<(), DomainError> {
        self.events.lock().unwrap().push(event.clone());
        Ok(())
    }
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<IngestedEvent>, DomainError> {
        Ok(self.events.lock().unwrap().iter().filter(|e| e.tenant_id == tenant_id).cloned().collect())
    }
}

#[derive(Default)]
struct MockFiringRepository {
    firings: Mutex<Vec<RuleFiring>>,
}
#[async_trait]
impl RuleFiringRepositoryPort for MockFiringRepository {
    async fn save(&self, firing: &RuleFiring) -> Result<(), DomainError> {
        self.firings.lock().unwrap().push(firing.clone());
        Ok(())
    }
    async fn find_by_event_id(&self, event_id: Uuid) -> Result<Vec<RuleFiring>, DomainError> {
        Ok(self.firings.lock().unwrap().iter().filter(|f| f.event_id == event_id).cloned().collect())
    }
}

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
    async fn update(&self, _rule: &AutomationRule) -> Result<(), DomainError> { Ok(()) }
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
    async fn delete_by_id(&self, _id: Uuid) -> Result<bool, DomainError> { Ok(false) }
}

#[derive(Default)]
struct MockPluginRepository {
    plugins: Mutex<Vec<Plugin>>,
}
#[async_trait]
impl PluginRepositoryPort for MockPluginRepository {
    async fn save(&self, plugin: &Plugin) -> Result<(), DomainError> {
        self.plugins.lock().unwrap().push(plugin.clone());
        Ok(())
    }
    async fn update(&self, _plugin: &Plugin) -> Result<(), DomainError> { Ok(()) }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Plugin>, DomainError> {
        Ok(self.plugins.lock().unwrap().iter().find(|p| p.id == id).cloned())
    }
    async fn find_by_tenant_including_global(&self, _tenant_id: Uuid) -> Result<Vec<Plugin>, DomainError> { Ok(Vec::new()) }
    async fn delete_by_id(&self, _id: Uuid) -> Result<bool, DomainError> { Ok(false) }
}

struct MockWorkflowClient {
    known_key: String,
    known_id: Uuid,
}
#[async_trait]
impl WorkflowClientPort for MockWorkflowClient {
    async fn start_execution(&self, workflow_definition_id: Uuid, _tenant_id: Uuid, _context: serde_json::Value) -> Result<serde_json::Value, DomainError> {
        Ok(serde_json::json!({"status": "running", "workflow_definition_id": workflow_definition_id}))
    }
    async fn find_definition_id_by_key(&self, _tenant_id: Uuid, key: &str) -> Result<Uuid, DomainError> {
        if key == self.known_key {
            Ok(self.known_id)
        } else {
            Err(DomainError::NotFound(format!("no workflow definition for key {}", key)))
        }
    }
}

#[derive(Default)]
struct MockAgentsClient {
    /// `None` simulates the agent execution itself failing; `Some(text)` is the
    /// raw text the classifier agent "said".
    output: Mutex<Option<String>>,
}
#[async_trait]
impl AgentsClientPort for MockAgentsClient {
    async fn execute_agent(&self, _agent_id: Uuid, _tenant_id: Uuid, _input: String) -> Result<String, DomainError> {
        self.output.lock().unwrap().clone().ok_or_else(|| DomainError::InternalError("agent execution failed".into()))
    }
}

struct MockSdlcClient;
#[async_trait]
impl SdlcClientPort for MockSdlcClient {
    async fn start_run(&self, _parameters: serde_json::Value) -> Result<serde_json::Value, DomainError> {
        Ok(serde_json::json!({"status": "completed"}))
    }
}

#[derive(Default)]
struct MockToolsClient {
    calls: AtomicU32,
    should_fail: bool,
}
#[async_trait]
impl AutomationToolsClientPort for MockToolsClient {
    async fn execute_tool(&self, _tool_id: Uuid, _action: String, _parameters: HashMap<String, String>) -> Result<serde_json::Value, DomainError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self.should_fail {
            Err(DomainError::InternalError("tool unreachable".into()))
        } else {
            Ok(serde_json::json!({"success": true}))
        }
    }
}

struct MockPluginDispatch;
#[async_trait]
impl PluginDispatchPort for MockPluginDispatch {
    async fn dispatch(&self, _plugin: &Plugin, _event_type: &str, _payload: &serde_json::Value, _parameters: &serde_json::Value) -> Result<serde_json::Value, DomainError> {
        Ok(serde_json::json!({"delivered": true}))
    }
}

fn rule(tenant_id: Uuid, event_type: &str, match_criteria: Option<&str>, action: &str) -> AutomationRule {
    AutomationRule::new(NewAutomationRule {
        tenant_id,
        name: "test rule".into(),
        event_type: event_type.into(),
        match_criteria: match_criteria.map(String::from),
        action: action.into(),
    })
}

struct Fixture {
    service: EventAutomationService,
    firing_repo: Arc<MockFiringRepository>,
    rule_repo: Arc<MockRuleRepository>,
    tools_client: Arc<MockToolsClient>,
    agents_client: Arc<MockAgentsClient>,
    known_workflow_key: String,
    known_workflow_id: Uuid,
}

fn fixture(tools_should_fail: bool) -> Fixture {
    let event_repo = Arc::new(MockEventRepository::default());
    let firing_repo = Arc::new(MockFiringRepository::default());
    let rule_repo = Arc::new(MockRuleRepository::default());
    let plugin_repo = Arc::new(MockPluginRepository::default());
    let tools_client = Arc::new(MockToolsClient { calls: AtomicU32::new(0), should_fail: tools_should_fail });
    let agents_client = Arc::new(MockAgentsClient::default());
    let known_workflow_key = "bug-fix-pipeline".to_string();
    let known_workflow_id = Uuid::new_v4();
    let workflow_client = Arc::new(MockWorkflowClient { known_key: known_workflow_key.clone(), known_id: known_workflow_id });
    let service = EventAutomationService::new(
        event_repo, firing_repo.clone(), rule_repo.clone(), plugin_repo,
        workflow_client, Arc::new(MockSdlcClient), tools_client.clone(), Arc::new(MockPluginDispatch),
        agents_client.clone(),
    );
    Fixture { service, firing_repo, rule_repo, tools_client, agents_client, known_workflow_key, known_workflow_id }
}

#[tokio::test]
async fn ingest_event_with_no_matching_rules_produces_no_firings() {
    let fx = fixture(false);
    let tenant_id = Uuid::new_v4();
    let response = fx.service.ingest_event(IngestEventCommand {
        tenant_id, event_type: "jira.ticket.transitioned".into(), payload: serde_json::json!({}),
    }).await.unwrap();
    assert_eq!(response.firings.len(), 0);
}

#[tokio::test]
async fn ingest_event_dispatches_action_when_criteria_match() {
    let fx = fixture(false);
    let tenant_id = Uuid::new_v4();
    fx.rule_repo.rules.lock().unwrap().push(rule(
        tenant_id, "jira.ticket.transitioned", Some(r#"{"status":"Ready for Dev"}"#),
        r#"{"action_type":"execute_tool","tool_id":"00000000-0000-0000-0000-000000000000","action":"build","parameters":{}}"#,
    ));

    let response = fx.service.ingest_event(IngestEventCommand {
        tenant_id, event_type: "jira.ticket.transitioned".into(), payload: serde_json::json!({"status": "Ready for Dev"}),
    }).await.unwrap();

    assert_eq!(response.firings.len(), 1);
    assert_eq!(response.firings[0].status, "succeeded");
    assert_eq!(fx.tools_client.calls.load(Ordering::SeqCst), 1);
    assert_eq!(fx.firing_repo.firings.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn ingest_event_skips_action_when_criteria_do_not_match() {
    let fx = fixture(false);
    let tenant_id = Uuid::new_v4();
    fx.rule_repo.rules.lock().unwrap().push(rule(
        tenant_id, "jira.ticket.transitioned", Some(r#"{"status":"Ready for Dev"}"#),
        r#"{"action_type":"execute_tool","tool_id":"00000000-0000-0000-0000-000000000000","action":"build","parameters":{}}"#,
    ));

    let response = fx.service.ingest_event(IngestEventCommand {
        tenant_id, event_type: "jira.ticket.transitioned".into(), payload: serde_json::json!({"status": "Blocked"}),
    }).await.unwrap();

    assert_eq!(response.firings.len(), 1);
    assert_eq!(response.firings[0].status, "skipped");
    assert!(!response.firings[0].matched);
    assert_eq!(fx.tools_client.calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn ingest_event_records_failure_without_failing_the_whole_call() {
    let fx = fixture(true);
    let tenant_id = Uuid::new_v4();
    fx.rule_repo.rules.lock().unwrap().push(rule(
        tenant_id, "manual", None,
        r#"{"action_type":"execute_tool","tool_id":"00000000-0000-0000-0000-000000000000","action":"build","parameters":{}}"#,
    ));

    let response = fx.service.ingest_event(IngestEventCommand {
        tenant_id, event_type: "manual".into(), payload: serde_json::json!({}),
    }).await.unwrap();

    assert_eq!(response.firings.len(), 1);
    assert_eq!(response.firings[0].status, "failed");
    assert!(response.firings[0].error.is_some());
}

#[tokio::test]
async fn ingest_event_fails_when_event_type_is_empty() {
    let fx = fixture(false);
    let result = fx.service.ingest_event(IngestEventCommand {
        tenant_id: Uuid::new_v4(), event_type: "".into(), payload: serde_json::json!({}),
    }).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

fn classify_and_dispatch_action() -> String {
    serde_json::json!({
        "action_type": "classify_and_dispatch",
        "classifier_agent_id": "00000000-0000-0000-0000-000000000000",
        "fallback_action": {"action_type": "start_sdlc_run", "parameters": {}},
    }).to_string()
}

#[tokio::test]
async fn classify_and_dispatch_routes_to_the_classified_workflow() {
    let fx = fixture(false);
    let tenant_id = Uuid::new_v4();
    *fx.agents_client.output.lock().unwrap() = Some(format!(
        r#"{{"workflow_key": "{}", "task_type": "bug"}}"#, fx.known_workflow_key
    ));
    fx.rule_repo.rules.lock().unwrap().push(rule(
        tenant_id, "jira.ticket.created", None, &classify_and_dispatch_action(),
    ));

    let response = fx.service.ingest_event(IngestEventCommand {
        tenant_id, event_type: "jira.ticket.created".into(), payload: serde_json::json!({"ticket_key": "PROJ-1"}),
    }).await.unwrap();

    assert_eq!(response.firings.len(), 1);
    assert_eq!(response.firings[0].status, "succeeded");
    let result: serde_json::Value = serde_json::from_str(response.firings[0].action_result.as_ref().unwrap()).unwrap();
    assert_eq!(result["workflow_definition_id"], serde_json::json!(fx.known_workflow_id));
}

#[tokio::test]
async fn classify_and_dispatch_falls_back_when_classifier_output_is_unusable() {
    let fx = fixture(false);
    let tenant_id = Uuid::new_v4();
    *fx.agents_client.output.lock().unwrap() = Some("I couldn't decide, sorry.".into());
    fx.rule_repo.rules.lock().unwrap().push(rule(
        tenant_id, "jira.ticket.created", None, &classify_and_dispatch_action(),
    ));

    let response = fx.service.ingest_event(IngestEventCommand {
        tenant_id, event_type: "jira.ticket.created".into(), payload: serde_json::json!({"ticket_key": "PROJ-1"}),
    }).await.unwrap();

    assert_eq!(response.firings.len(), 1);
    assert_eq!(response.firings[0].status, "succeeded");
    let result: serde_json::Value = serde_json::from_str(response.firings[0].action_result.as_ref().unwrap()).unwrap();
    assert_eq!(result["status"], "completed");
}

#[tokio::test]
async fn classify_and_dispatch_falls_back_when_workflow_key_has_no_definition() {
    let fx = fixture(false);
    let tenant_id = Uuid::new_v4();
    *fx.agents_client.output.lock().unwrap() = Some(r#"{"workflow_key": "no-such-key", "task_type": "bug"}"#.into());
    fx.rule_repo.rules.lock().unwrap().push(rule(
        tenant_id, "jira.ticket.created", None, &classify_and_dispatch_action(),
    ));

    let response = fx.service.ingest_event(IngestEventCommand {
        tenant_id, event_type: "jira.ticket.created".into(), payload: serde_json::json!({"ticket_key": "PROJ-1"}),
    }).await.unwrap();

    assert_eq!(response.firings.len(), 1);
    assert_eq!(response.firings[0].status, "succeeded");
    let result: serde_json::Value = serde_json::from_str(response.firings[0].action_result.as_ref().unwrap()).unwrap();
    assert_eq!(result["status"], "completed");
}
