use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;

use sdlc_dataaccess::node_executor::ToolNodeExecutor::ToolNodeExecutor;
use sdlc_domain::dto::WorkflowGraph::{NodeType, WorkflowNode};
use sdlc_domain::port::output::AutomationToolsClientPort::AutomationToolsClientPort;
use sdlc_domain::port::output::NodeExecutorPort::NodeExecutorPort;
use sdlc_domain::r#enum::DomainError::DomainError;

struct MockToolsClient;

#[async_trait]
impl AutomationToolsClientPort for MockToolsClient {
    async fn execute_tool(&self, _tool_id: Uuid, action: String, parameters: HashMap<String, String>) -> Result<serde_json::Value, DomainError> {
        Ok(serde_json::json!({ "action": action, "parameters": parameters }))
    }
}

fn task_node(config: Option<serde_json::Value>) -> WorkflowNode {
    WorkflowNode {
        id: "tool-node".into(),
        name: "Tool Node".into(),
        node_type: NodeType::Task,
        executor: Some("tool".into()),
        retry_policy: None,
        join: false,
        config,
    }
}

#[tokio::test]
async fn execute_runs_the_configured_tool_action_and_namespaces_output() {
    let executor = ToolNodeExecutor::new(Arc::new(MockToolsClient));
    let node = task_node(Some(serde_json::json!({
        "tool_id": Uuid::new_v4(),
        "action": "build",
        "parameters": {"target": "release"},
    })));

    let output = executor.execute(&node, Uuid::new_v4(), &serde_json::json!({})).await.unwrap();

    assert_eq!(output["tool-node_output"]["action"], "build");
    assert_eq!(output["tool-node_output"]["parameters"]["target"], "release");
}

#[tokio::test]
async fn execute_fails_when_action_is_missing() {
    let executor = ToolNodeExecutor::new(Arc::new(MockToolsClient));
    let node = task_node(Some(serde_json::json!({ "tool_id": Uuid::new_v4() })));

    let result = executor.execute(&node, Uuid::new_v4(), &serde_json::json!({})).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn execute_fails_when_config_is_missing() {
    let executor = ToolNodeExecutor::new(Arc::new(MockToolsClient));
    let node = task_node(None);

    let result = executor.execute(&node, Uuid::new_v4(), &serde_json::json!({})).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}
