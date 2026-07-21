use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use chrono::Utc;
use tracing::{error, info, warn};
use uuid::Uuid;
use sdlc_domain::dto::ApprovalDecisionCommand::ApprovalDecisionCommand;
use sdlc_domain::dto::ExecutionStatus::ExecutionStatus;
use sdlc_domain::dto::NodeExecutionStatus::NodeExecutionStatus;
use sdlc_domain::dto::StartWorkflowExecutionCommand::StartWorkflowExecutionCommand;
use sdlc_domain::dto::WorkflowExecution::{NewWorkflowExecution, WorkflowExecution};
use sdlc_domain::dto::WorkflowExecutionResponse::WorkflowExecutionResponse;
use sdlc_domain::dto::WorkflowGraph::{NodeType, WorkflowGraph, WorkflowNode};
use sdlc_domain::dto::WorkflowNodeExecution::{NewWorkflowNodeExecution, WorkflowNodeExecution};
use sdlc_domain::dto::WorkflowNodeExecutionResponse::WorkflowNodeExecutionResponse;
use sdlc_domain::port::input::WorkflowExecutionPort::WorkflowExecutionPort;
use sdlc_domain::port::output::NodeExecutorPort::NodeExecutorPort;
use sdlc_domain::port::output::WorkflowDefinitionRepositoryPort::WorkflowDefinitionRepositoryPort;
use sdlc_domain::port::output::WorkflowExecutionRepositoryPort::WorkflowExecutionRepositoryPort;
use sdlc_domain::port::output::WorkflowNodeExecutionRepositoryPort::WorkflowNodeExecutionRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::WorkflowEngineService::WorkflowEngineService;

use sdlc_domain::dto::WorkflowDefinition::{NewWorkflowDefinition, WorkflowDefinition};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

#[derive(Default)]
struct MockDefinitionRepository {
    definitions: Mutex<Vec<WorkflowDefinition>>,
}

#[async_trait]
impl WorkflowDefinitionRepositoryPort for MockDefinitionRepository {
    async fn save(&self, definition: &WorkflowDefinition) -> Result<(), DomainError> {
        self.definitions.lock().unwrap().push(definition.clone());
        Ok(())
    }
    async fn update(&self, _definition: &WorkflowDefinition) -> Result<(), DomainError> {
        Ok(())
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<WorkflowDefinition>, DomainError> {
        Ok(self.definitions.lock().unwrap().iter().find(|d| d.id == id).cloned())
    }
    async fn find_all_by_key_and_tenant(&self, _tenant_id: Uuid, _key: &str) -> Result<Vec<WorkflowDefinition>, DomainError> {
        Ok(Vec::new())
    }
    async fn find_by_tenant(&self, _tenant_id: Uuid) -> Result<Vec<WorkflowDefinition>, DomainError> {
        Ok(Vec::new())
    }
    async fn delete_by_id(&self, _id: Uuid) -> Result<bool, DomainError> {
        Ok(false)
    }
}

#[derive(Default)]
struct MockExecutionRepository {
    executions: Mutex<HashMap<Uuid, WorkflowExecution>>,
}

#[async_trait]
impl WorkflowExecutionRepositoryPort for MockExecutionRepository {
    async fn save(&self, execution: &WorkflowExecution) -> Result<(), DomainError> {
        self.executions.lock().unwrap().insert(execution.id, execution.clone());
        Ok(())
    }
    async fn update(&self, execution: &WorkflowExecution) -> Result<(), DomainError> {
        self.executions.lock().unwrap().insert(execution.id, execution.clone());
        Ok(())
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<WorkflowExecution>, DomainError> {
        Ok(self.executions.lock().unwrap().get(&id).cloned())
    }
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WorkflowExecution>, DomainError> {
        Ok(self.executions.lock().unwrap().values().filter(|e| e.tenant_id == tenant_id).cloned().collect())
    }
}

#[derive(Default)]
struct MockNodeExecutionRepository {
    rows: Mutex<Vec<WorkflowNodeExecution>>,
}

#[async_trait]
impl WorkflowNodeExecutionRepositoryPort for MockNodeExecutionRepository {
    async fn save(&self, node_execution: &WorkflowNodeExecution) -> Result<(), DomainError> {
        self.rows.lock().unwrap().push(node_execution.clone());
        Ok(())
    }
    async fn update(&self, node_execution: &WorkflowNodeExecution) -> Result<(), DomainError> {
        let mut rows = self.rows.lock().unwrap();
        if let Some(existing) = rows.iter_mut().find(|r| r.id == node_execution.id) {
            *existing = node_execution.clone();
        }
        Ok(())
    }
    async fn find_by_execution_id(&self, execution_id: Uuid) -> Result<Vec<WorkflowNodeExecution>, DomainError> {
        Ok(self.rows.lock().unwrap().iter().filter(|r| r.workflow_execution_id == execution_id).cloned().collect())
    }
    async fn find_latest_by_execution_and_node(&self, execution_id: Uuid, node_id: &str) -> Result<Option<WorkflowNodeExecution>, DomainError> {
        Ok(self.rows.lock().unwrap().iter()
            .filter(|r| r.workflow_execution_id == execution_id && r.node_id == node_id)
            .max_by_key(|r| r.attempt)
            .cloned())
    }
}

struct NoopExecutor;
#[async_trait]
impl NodeExecutorPort for NoopExecutor {
    fn executor_name(&self) -> &'static str { "noop" }
    async fn execute(&self, _node: &WorkflowNode, _tenant_id: Uuid, _context: &serde_json::Value) -> Result<serde_json::Value, DomainError> {
        Ok(serde_json::json!({}))
    }
}

struct AlwaysFailExecutor;
#[async_trait]
impl NodeExecutorPort for AlwaysFailExecutor {
    fn executor_name(&self) -> &'static str { "always_fail" }
    async fn execute(&self, _node: &WorkflowNode, _tenant_id: Uuid, _context: &serde_json::Value) -> Result<serde_json::Value, DomainError> {
        Err(DomainError::InternalError("boom".into()))
    }
}

/// Fails the first `fail_times` calls, then succeeds — used to exercise retries.
struct FlakyExecutor {
    fail_times: u32,
    calls: AtomicU32,
}
#[async_trait]
impl NodeExecutorPort for FlakyExecutor {
    fn executor_name(&self) -> &'static str { "flaky" }
    async fn execute(&self, _node: &WorkflowNode, _tenant_id: Uuid, _context: &serde_json::Value) -> Result<serde_json::Value, DomainError> {
        let n = self.calls.fetch_add(1, Ordering::SeqCst);
        if n < self.fail_times {
            Err(DomainError::InternalError("transient".into()))
        } else {
            Ok(serde_json::json!({"recovered": true}))
        }
    }
}

/// Always returns a fixed single-field JSON object — used to feed values into
/// the execution context so downstream edge conditions can branch on them.
struct OutputExecutor {
    field: &'static str,
    value: serde_json::Value,
}
#[async_trait]
impl NodeExecutorPort for OutputExecutor {
    fn executor_name(&self) -> &'static str { "output" }
    async fn execute(&self, _node: &WorkflowNode, _tenant_id: Uuid, _context: &serde_json::Value) -> Result<serde_json::Value, DomainError> {
        let mut map = serde_json::Map::new();
        map.insert(self.field.to_string(), self.value.clone());
        Ok(serde_json::Value::Object(map))
    }
}

fn make_definition(tenant_id: Uuid, definition_json: &str) -> WorkflowDefinition {
    WorkflowDefinition::new(NewWorkflowDefinition {
        tenant_id,
        key: "test".into(),
        version: 1,
        name: "Test".into(),
        description: None,
        definition: definition_json.into(),
    })
}

struct Fixture {
    engine: WorkflowEngineService,
    definition_id: Uuid,
    tenant_id: Uuid,
    node_repo: Arc<MockNodeExecutionRepository>,
    exec_repo: Arc<MockExecutionRepository>,
    def_repo: Arc<MockDefinitionRepository>,
}

fn build_fixture(definition_json: &str, executors: Vec<Arc<dyn NodeExecutorPort>>) -> Fixture {
    let tenant_id = Uuid::new_v4();
    let definition = make_definition(tenant_id, definition_json);
    let definition_id = definition.id;

    let def_repo = Arc::new(MockDefinitionRepository::default());
    def_repo.definitions.lock().unwrap().push(definition);
    let exec_repo = Arc::new(MockExecutionRepository::default());
    let node_repo = Arc::new(MockNodeExecutionRepository::default());

    let engine = WorkflowEngineService::new(def_repo.clone(), exec_repo.clone(), node_repo.clone(), executors);
    Fixture { engine, definition_id, tenant_id, node_repo, exec_repo, def_repo }
}

#[tokio::test]
async fn linear_graph_completes_successfully() {
    let graph = r#"{
        "nodes": [
            {"id":"start","name":"Start","node_type":"start"},
            {"id":"task","name":"Task","node_type":"task"},
            {"id":"end","name":"End","node_type":"end"}
        ],
        "edges": [
            {"from":"start","to":"task"},
            {"from":"task","to":"end"}
        ]
    }"#;
    let fixture = build_fixture(graph, vec![Arc::new(NoopExecutor)]);
    let response = fixture.engine.start_execution(StartWorkflowExecutionCommand {
        tenant_id: fixture.tenant_id,
        workflow_definition_id: fixture.definition_id,
        context: None,
    }).await.unwrap();
    assert_eq!(response.status, "completed");
}

#[tokio::test]
async fn conditional_edge_only_runs_the_matching_branch() {
    let graph = r#"{
        "nodes": [
            {"id":"start","name":"Start","node_type":"start"},
            {"id":"check","name":"Check","node_type":"task","executor":"output"},
            {"id":"task_a","name":"A","node_type":"task"},
            {"id":"task_b","name":"B","node_type":"task"},
            {"id":"end","name":"End","node_type":"end"}
        ],
        "edges": [
            {"from":"start","to":"check"},
            {"from":"check","to":"task_a","condition":"passed == true"},
            {"from":"check","to":"task_b","condition":"passed == false"},
            {"from":"task_a","to":"end"},
            {"from":"task_b","to":"end"}
        ]
    }"#;
    let executors: Vec<Arc<dyn NodeExecutorPort>> = vec![
        Arc::new(NoopExecutor),
        Arc::new(OutputExecutor { field: "passed", value: serde_json::json!(true) }),
    ];
    let fixture = build_fixture(graph, executors);
    let response = fixture.engine.start_execution(StartWorkflowExecutionCommand {
        tenant_id: fixture.tenant_id,
        workflow_definition_id: fixture.definition_id,
        context: None,
    }).await.unwrap();
    assert_eq!(response.status, "completed");

    let rows = fixture.node_repo.rows.lock().unwrap();
    assert!(rows.iter().any(|r| r.node_id == "task_a" && r.status == NodeExecutionStatus::Succeeded));
    assert!(!rows.iter().any(|r| r.node_id == "task_b"));
}

#[tokio::test]
async fn parallel_fan_out_joins_before_continuing() {
    let graph = r#"{
        "nodes": [
            {"id":"start","name":"Start","node_type":"start"},
            {"id":"task_a","name":"A","node_type":"task"},
            {"id":"task_b","name":"B","node_type":"task"},
            {"id":"join","name":"Join","node_type":"task","join":true},
            {"id":"end","name":"End","node_type":"end"}
        ],
        "edges": [
            {"from":"start","to":"task_a"},
            {"from":"start","to":"task_b"},
            {"from":"task_a","to":"join"},
            {"from":"task_b","to":"join"},
            {"from":"join","to":"end"}
        ]
    }"#;
    let fixture = build_fixture(graph, vec![Arc::new(NoopExecutor)]);
    let response = fixture.engine.start_execution(StartWorkflowExecutionCommand {
        tenant_id: fixture.tenant_id,
        workflow_definition_id: fixture.definition_id,
        context: None,
    }).await.unwrap();
    assert_eq!(response.status, "completed");

    let rows = fixture.node_repo.rows.lock().unwrap();
    for node_id in ["start", "task_a", "task_b", "join", "end"] {
        assert!(rows.iter().any(|r| r.node_id == node_id && r.status == NodeExecutionStatus::Succeeded), "missing {}", node_id);
    }
}

#[tokio::test]
async fn task_retries_until_it_succeeds() {
    let graph = r#"{
        "nodes": [
            {"id":"start","name":"Start","node_type":"start"},
            {"id":"flaky_task","name":"Flaky","node_type":"task","executor":"flaky","retry_policy":{"max_attempts":3,"backoff_seconds":0}},
            {"id":"end","name":"End","node_type":"end"}
        ],
        "edges": [
            {"from":"start","to":"flaky_task"},
            {"from":"flaky_task","to":"end"}
        ]
    }"#;
    let executors: Vec<Arc<dyn NodeExecutorPort>> = vec![Arc::new(FlakyExecutor { fail_times: 2, calls: AtomicU32::new(0) })];
    let fixture = build_fixture(graph, executors);
    let response = fixture.engine.start_execution(StartWorkflowExecutionCommand {
        tenant_id: fixture.tenant_id,
        workflow_definition_id: fixture.definition_id,
        context: None,
    }).await.unwrap();
    assert_eq!(response.status, "completed");

    let rows = fixture.node_repo.rows.lock().unwrap();
    let attempts: Vec<i32> = rows.iter().filter(|r| r.node_id == "flaky_task").map(|r| r.attempt).collect();
    assert_eq!(attempts.len(), 3);
    let last = rows.iter().filter(|r| r.node_id == "flaky_task").max_by_key(|r| r.attempt).unwrap();
    assert_eq!(last.status, NodeExecutionStatus::Succeeded);
}

#[tokio::test]
async fn task_fails_the_execution_once_retries_are_exhausted() {
    let graph = r#"{
        "nodes": [
            {"id":"start","name":"Start","node_type":"start"},
            {"id":"failing_task","name":"Failing","node_type":"task","executor":"always_fail","retry_policy":{"max_attempts":2,"backoff_seconds":0}},
            {"id":"end","name":"End","node_type":"end"}
        ],
        "edges": [
            {"from":"start","to":"failing_task"},
            {"from":"failing_task","to":"end"}
        ]
    }"#;
    let fixture = build_fixture(graph, vec![Arc::new(AlwaysFailExecutor)]);
    let response = fixture.engine.start_execution(StartWorkflowExecutionCommand {
        tenant_id: fixture.tenant_id,
        workflow_definition_id: fixture.definition_id,
        context: None,
    }).await.unwrap();
    assert_eq!(response.status, "failed");
    assert!(response.error.is_some());

    let rows = fixture.node_repo.rows.lock().unwrap();
    assert_eq!(rows.iter().filter(|r| r.node_id == "failing_task").count(), 2);
}

fn approval_graph() -> &'static str {
    r#"{
        "nodes": [
            {"id":"start","name":"Start","node_type":"start"},
            {"id":"gate","name":"Gate","node_type":"approval"},
            {"id":"end","name":"End","node_type":"end"}
        ],
        "edges": [
            {"from":"start","to":"gate"},
            {"from":"gate","to":"end"}
        ]
    }"#
}

#[tokio::test]
async fn approval_node_pauses_and_resumes_on_approve() {
    let fixture = build_fixture(approval_graph(), vec![Arc::new(NoopExecutor)]);
    let started = fixture.engine.start_execution(StartWorkflowExecutionCommand {
        tenant_id: fixture.tenant_id,
        workflow_definition_id: fixture.definition_id,
        context: None,
    }).await.unwrap();
    assert_eq!(started.status, "waiting_approval");

    let resumed = fixture.engine.decide_approval(started.id, "gate", ApprovalDecisionCommand {
        approved: true,
        comment: Some("looks good".into()),
    }).await.unwrap();
    assert_eq!(resumed.status, "completed");
}

#[tokio::test]
async fn approval_node_fails_the_execution_on_reject() {
    let fixture = build_fixture(approval_graph(), vec![Arc::new(NoopExecutor)]);
    let started = fixture.engine.start_execution(StartWorkflowExecutionCommand {
        tenant_id: fixture.tenant_id,
        workflow_definition_id: fixture.definition_id,
        context: None,
    }).await.unwrap();

    let resumed = fixture.engine.decide_approval(started.id, "gate", ApprovalDecisionCommand {
        approved: false,
        comment: None,
    }).await.unwrap();
    assert_eq!(resumed.status, "failed");
    assert!(resumed.error.unwrap().contains("Rejected"));
}

#[tokio::test]
async fn resume_works_from_a_brand_new_engine_sharing_only_the_repositories() {
    // Demonstrates that progress is a property of the persisted checkpoint rows,
    // not of any in-memory engine state: a second engine instance backed by the
    // same repositories can resume an execution it never started.
    let fixture = build_fixture(approval_graph(), vec![Arc::new(NoopExecutor)]);
    let started = fixture.engine.start_execution(StartWorkflowExecutionCommand {
        tenant_id: fixture.tenant_id,
        workflow_definition_id: fixture.definition_id,
        context: None,
    }).await.unwrap();
    assert_eq!(started.status, "waiting_approval");

    let other_engine = WorkflowEngineService::new(
        fixture.def_repo.clone(),
        fixture.exec_repo.clone(),
        fixture.node_repo.clone(),
        vec![Arc::new(NoopExecutor)],
    );
    let resumed = other_engine.decide_approval(started.id, "gate", ApprovalDecisionCommand {
        approved: true,
        comment: None,
    }).await.unwrap();
    assert_eq!(resumed.status, "completed");
}
