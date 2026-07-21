use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::WorkflowGraph::WorkflowNode;
use crate::r#enum::DomainError::DomainError;

/// A driven adapter that carries out the work for one `task` node. Implementations
/// live in `sdlc-dataaccess/src/node_executor/` and are looked up by name (see
/// `WorkflowNode::executor`, default `"noop"`). `agent` and `tool` call out to the
/// Agent Runtime and Tools service respectively; the engine itself (retries,
/// checkpoints, parallel scheduling) is unaffected by which executor runs.
#[async_trait]
pub trait NodeExecutorPort: Send + Sync {
    fn executor_name(&self) -> &'static str;

    /// Returns the node's JSON output on success, which the engine shallow-merges
    /// into the execution context for downstream edge conditions to reference.
    async fn execute(&self, node: &WorkflowNode, tenant_id: Uuid, context: &serde_json::Value) -> Result<serde_json::Value, DomainError>;
}
