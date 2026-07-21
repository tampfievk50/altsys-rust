use async_trait::async_trait;
use tracing::info;
use uuid::Uuid;

use sdlc_domain::dto::WorkflowGraph::WorkflowNode;
use sdlc_domain::port::output::NodeExecutorPort::NodeExecutorPort;
use sdlc_domain::r#enum::DomainError::DomainError;

/// Default `task` executor (`executor: "noop"`, or omitted). Logs and immediately
/// succeeds with an empty output — exercises the engine's graph/retry/parallel/
/// checkpoint machinery without doing real work. See `AgentNodeExecutor` and
/// `ToolNodeExecutor` for executors that do real work.
pub struct NoOpNodeExecutor;

#[async_trait]
impl NodeExecutorPort for NoOpNodeExecutor {
    fn executor_name(&self) -> &'static str {
        "noop"
    }

    async fn execute(&self, node: &WorkflowNode, _tenant_id: Uuid, _context: &serde_json::Value) -> Result<serde_json::Value, DomainError> {
        info!(node_id = %node.id, node_name = %node.name, "Executing no-op task node");
        Ok(serde_json::json!({}))
    }
}
