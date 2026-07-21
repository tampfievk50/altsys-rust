use sdlc_domain::dto::WorkflowGraph::{NodeType, WorkflowEdge, WorkflowGraph, WorkflowNode};
use sdlc_domain::r#enum::DomainError::DomainError;

fn linear_graph() -> WorkflowGraph {
    WorkflowGraph {
        nodes: vec![
            WorkflowNode { id: "start".into(), name: "Start".into(), node_type: NodeType::Start, executor: None, retry_policy: None, join: false, config: None },
            WorkflowNode { id: "end".into(), name: "End".into(), node_type: NodeType::End, executor: None, retry_policy: None, join: false, config: None },
        ],
        edges: vec![WorkflowEdge { from: "start".into(), to: "end".into(), condition: None }],
    }
}

#[test]
fn validate_accepts_a_well_formed_graph() {
    assert!(linear_graph().validate().is_ok());
}

#[test]
fn validate_rejects_missing_start_node() {
    let mut graph = linear_graph();
    graph.nodes.retain(|n| n.node_type != NodeType::Start);
    assert!(matches!(graph.validate(), Err(DomainError::ValidationError(_))));
}

#[test]
fn validate_rejects_duplicate_node_ids() {
    let mut graph = linear_graph();
    graph.nodes.push(graph.nodes[0].clone());
    assert!(matches!(graph.validate(), Err(DomainError::ValidationError(_))));
}

#[test]
fn validate_rejects_edge_to_unknown_node() {
    let mut graph = linear_graph();
    graph.edges.push(WorkflowEdge { from: "start".into(), to: "missing".into(), condition: None });
    assert!(matches!(graph.validate(), Err(DomainError::ValidationError(_))));
}

#[test]
fn edge_without_condition_always_matches() {
    let edge = WorkflowEdge { from: "a".into(), to: "b".into(), condition: None };
    assert!(edge.matches(&serde_json::json!({})).unwrap());
}

#[test]
fn edge_condition_matches_boolean_context_field() {
    let edge = WorkflowEdge { from: "a".into(), to: "b".into(), condition: Some("tests_passed == true".into()) };
    assert!(edge.matches(&serde_json::json!({"tests_passed": true})).unwrap());
    assert!(!edge.matches(&serde_json::json!({"tests_passed": false})).unwrap());
    assert!(!edge.matches(&serde_json::json!({})).unwrap());
}

#[test]
fn edge_condition_matches_string_context_field() {
    let edge = WorkflowEdge { from: "a".into(), to: "b".into(), condition: Some(r#"status == "ready""#.into()) };
    assert!(edge.matches(&serde_json::json!({"status": "ready"})).unwrap());
    assert!(!edge.matches(&serde_json::json!({"status": "blocked"})).unwrap());
}
