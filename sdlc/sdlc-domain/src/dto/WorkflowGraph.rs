use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::r#enum::DomainError::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    Start,
    Task,
    Approval,
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    #[serde(default = "RetryPolicy::default_max_attempts")]
    pub max_attempts: u32,
    #[serde(default)]
    pub backoff_seconds: u64,
}

impl RetryPolicy {
    fn default_max_attempts() -> u32 {
        1
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNode {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,
    /// Names the `NodeExecutorPort` implementation used to run this node (`task` nodes only).
    /// Defaults to `"noop"`, the built-in executor.
    #[serde(default)]
    pub executor: Option<String>,
    #[serde(default)]
    pub retry_policy: Option<RetryPolicy>,
    /// When true, this node only becomes ready once *every* node with an edge leading
    /// into it has succeeded (AND-join). Default is OR-join: ready as soon as any one
    /// incoming edge fires. Use `join: true` for the "wait for all parallel branches"
    /// pattern; leave it `false` for ordinary conditional branching.
    #[serde(default)]
    pub join: bool,
    /// Opaque, executor-interpreted configuration (`task` nodes only) — e.g. the
    /// `agent` executor expects `{"agent_id": "...", "instruction": "..."}`, the
    /// `tool` executor expects `{"tool_id": "...", "action": "...", "parameters": {}}`.
    #[serde(default)]
    pub config: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowEdge {
    pub from: String,
    pub to: String,
    /// Optional `<field> == <value>` expression evaluated against the execution
    /// context. `value` may be `true`, `false`, a number, or a (optionally quoted)
    /// string. No condition means the edge always fires once `from` succeeds.
    #[serde(default)]
    pub condition: Option<String>,
}

impl WorkflowEdge {
    pub fn validate_condition_syntax(condition: &str) -> Result<(), DomainError> {
        let parts: Vec<&str> = condition.splitn(2, "==").collect();
        if parts.len() != 2 || parts[0].trim().is_empty() || parts[1].trim().is_empty() {
            return Err(DomainError::ValidationError(format!("Unsupported condition syntax: '{}'", condition)));
        }
        Ok(())
    }

    /// Evaluates `self.condition` against `context`. Missing conditions always match;
    /// a missing context field never matches.
    pub fn matches(&self, context: &serde_json::Value) -> Result<bool, DomainError> {
        let Some(condition) = &self.condition else {
            return Ok(true);
        };
        let parts: Vec<&str> = condition.splitn(2, "==").collect();
        if parts.len() != 2 {
            return Err(DomainError::ValidationError(format!("Unsupported condition syntax: '{}'", condition)));
        }
        let field = parts[0].trim();
        let expected_raw = parts[1].trim();
        let expected = if expected_raw == "true" {
            serde_json::Value::Bool(true)
        } else if expected_raw == "false" {
            serde_json::Value::Bool(false)
        } else if let Ok(n) = expected_raw.parse::<f64>() {
            serde_json::json!(n)
        } else {
            serde_json::Value::String(expected_raw.trim_matches('"').to_string())
        };
        let actual = context.get(field).cloned().unwrap_or(serde_json::Value::Null);
        Ok(actual == expected)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowGraph {
    pub nodes: Vec<WorkflowNode>,
    pub edges: Vec<WorkflowEdge>,
}

impl WorkflowGraph {
    pub fn parse(definition: &str) -> Result<Self, DomainError> {
        serde_json::from_str(definition).map_err(|e| DomainError::ValidationError(format!("Invalid workflow definition JSON: {}", e)))
    }

    pub fn start_node(&self) -> Option<&WorkflowNode> {
        self.nodes.iter().find(|n| n.node_type == NodeType::Start)
    }

    pub fn find_node(&self, id: &str) -> Option<&WorkflowNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    /// Structural validation: unique node ids, exactly one Start node, at least one
    /// End node, and every edge referencing a real node with well-formed conditions.
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.nodes.is_empty() {
            return Err(DomainError::ValidationError("Workflow must have at least one node".into()));
        }
        let ids: HashSet<&str> = self.nodes.iter().map(|n| n.id.as_str()).collect();
        if ids.len() != self.nodes.len() {
            return Err(DomainError::ValidationError("Node ids must be unique".into()));
        }
        let start_count = self.nodes.iter().filter(|n| n.node_type == NodeType::Start).count();
        if start_count != 1 {
            return Err(DomainError::ValidationError(format!("Workflow must have exactly one Start node, found {}", start_count)));
        }
        if !self.nodes.iter().any(|n| n.node_type == NodeType::End) {
            return Err(DomainError::ValidationError("Workflow must have at least one End node".into()));
        }
        for edge in &self.edges {
            if !ids.contains(edge.from.as_str()) {
                return Err(DomainError::ValidationError(format!("Edge references unknown node '{}'", edge.from)));
            }
            if !ids.contains(edge.to.as_str()) {
                return Err(DomainError::ValidationError(format!("Edge references unknown node '{}'", edge.to)));
            }
            if let Some(condition) = &edge.condition {
                WorkflowEdge::validate_condition_syntax(condition)?;
            }
        }
        Ok(())
    }
}
