use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;

use sdlc_dataaccess::node_executor::AgentNodeExecutor::AgentNodeExecutor;
use sdlc_domain::dto::WorkflowGraph::{NodeType, WorkflowNode};
use sdlc_domain::port::output::AgentsClientPort::AgentsClientPort;
use sdlc_domain::port::output::NodeExecutorPort::NodeExecutorPort;
use sdlc_domain::r#enum::DomainError::DomainError;

struct MockAgentsClient {
    response: Result<String, String>,
}

#[async_trait]
impl AgentsClientPort for MockAgentsClient {
    async fn execute_agent(&self, _agent_id: Uuid, _tenant_id: Uuid, _input: String) -> Result<String, DomainError> {
        self.response.clone().map_err(DomainError::InternalError)
    }
}

fn task_node(config: Option<serde_json::Value>) -> WorkflowNode {
    WorkflowNode {
        id: "agent-node".into(),
        name: "Agent Node".into(),
        node_type: NodeType::Task,
        executor: Some("agent".into()),
        retry_policy: None,
        join: false,
        config,
    }
}

#[tokio::test]
async fn execute_calls_the_configured_agent_and_namespaces_output() {
    let agents_client = Arc::new(MockAgentsClient { response: Ok("looks good".into()) });
    let executor = AgentNodeExecutor::new(agents_client);
    let node = task_node(Some(serde_json::json!({ "agent_id": Uuid::new_v4(), "instruction": "Review this." })));

    let output = executor.execute(&node, Uuid::new_v4(), &serde_json::json!({"ticket_key": "PROJ-1"})).await.unwrap();

    assert_eq!(output["agent-node_output"], "looks good");
}

#[tokio::test]
async fn execute_fails_when_config_is_missing() {
    let executor = AgentNodeExecutor::new(Arc::new(MockAgentsClient { response: Ok("unused".into()) }));
    let node = task_node(None);

    let result = executor.execute(&node, Uuid::new_v4(), &serde_json::json!({})).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn execute_fails_when_agent_id_is_invalid() {
    let executor = AgentNodeExecutor::new(Arc::new(MockAgentsClient { response: Ok("unused".into()) }));
    let node = task_node(Some(serde_json::json!({ "agent_id": "not-a-uuid" })));

    let result = executor.execute(&node, Uuid::new_v4(), &serde_json::json!({})).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn execute_surfaces_agent_failure() {
    let executor = AgentNodeExecutor::new(Arc::new(MockAgentsClient { response: Err("agent exploded".into()) }));
    let node = task_node(Some(serde_json::json!({ "agent_id": Uuid::new_v4() })));

    let result = executor.execute(&node, Uuid::new_v4(), &serde_json::json!({})).await;
    assert!(result.is_err());
}
