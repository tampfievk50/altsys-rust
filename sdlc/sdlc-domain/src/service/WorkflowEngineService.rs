use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::dto::ApprovalDecisionCommand::ApprovalDecisionCommand;
use crate::dto::ExecutionStatus::ExecutionStatus;
use crate::dto::NodeExecutionStatus::NodeExecutionStatus;
use crate::dto::StartWorkflowExecutionCommand::StartWorkflowExecutionCommand;
use crate::dto::WorkflowExecution::{NewWorkflowExecution, WorkflowExecution};
use crate::dto::WorkflowExecutionResponse::WorkflowExecutionResponse;
use crate::dto::WorkflowGraph::{NodeType, WorkflowGraph, WorkflowNode};
use crate::dto::WorkflowNodeExecution::{NewWorkflowNodeExecution, WorkflowNodeExecution};
use crate::dto::WorkflowNodeExecutionResponse::WorkflowNodeExecutionResponse;
use crate::port::input::WorkflowExecutionPort::WorkflowExecutionPort;
use crate::port::output::NodeExecutorPort::NodeExecutorPort;
use crate::port::output::WorkflowDefinitionRepositoryPort::WorkflowDefinitionRepositoryPort;
use crate::port::output::WorkflowExecutionRepositoryPort::WorkflowExecutionRepositoryPort;
use crate::port::output::WorkflowNodeExecutionRepositoryPort::WorkflowNodeExecutionRepositoryPort;
use crate::r#enum::DomainError::DomainError;

const DEFAULT_EXECUTOR: &str = "noop";
/// Guards against a malformed graph (e.g. an accidental cycle with no terminal
/// condition) turning into an infinite scheduling loop.
const MAX_SCHEDULING_ROUNDS: usize = 1000;

pub struct WorkflowEngineService {
    definition_repository: Arc<dyn WorkflowDefinitionRepositoryPort>,
    execution_repository: Arc<dyn WorkflowExecutionRepositoryPort>,
    node_execution_repository: Arc<dyn WorkflowNodeExecutionRepositoryPort>,
    executors: HashMap<String, Arc<dyn NodeExecutorPort>>,
}

impl WorkflowEngineService {
    pub fn new(
        definition_repository: Arc<dyn WorkflowDefinitionRepositoryPort>,
        execution_repository: Arc<dyn WorkflowExecutionRepositoryPort>,
        node_execution_repository: Arc<dyn WorkflowNodeExecutionRepositoryPort>,
        executors: Vec<Arc<dyn NodeExecutorPort>>,
    ) -> Self {
        let executors = executors.into_iter().map(|e| (e.executor_name().to_string(), e)).collect();
        Self { definition_repository, execution_repository, node_execution_repository, executors }
    }

    fn to_response(execution: &WorkflowExecution) -> WorkflowExecutionResponse {
        WorkflowExecutionResponse {
            id: execution.id,
            tenant_id: execution.tenant_id,
            workflow_definition_id: execution.workflow_definition_id,
            status: execution.status.to_string(),
            context: execution.context.to_string(),
            error: execution.error.clone(),
            created_at: execution.created_at,
            updated_at: execution.updated_at,
            started_at: execution.started_at,
            completed_at: execution.completed_at,
        }
    }

    fn to_node_response(node_execution: &WorkflowNodeExecution) -> WorkflowNodeExecutionResponse {
        WorkflowNodeExecutionResponse {
            id: node_execution.id,
            workflow_execution_id: node_execution.workflow_execution_id,
            node_id: node_execution.node_id.clone(),
            status: node_execution.status.to_string(),
            attempt: node_execution.attempt,
            input: node_execution.input.as_ref().map(|v| v.to_string()),
            output: node_execution.output.as_ref().map(|v| v.to_string()),
            error: node_execution.error.clone(),
            started_at: node_execution.started_at,
            completed_at: node_execution.completed_at,
        }
    }

    async fn load_graph(&self, workflow_definition_id: Uuid) -> Result<WorkflowGraph, DomainError> {
        let definition = self.definition_repository.find_by_id(workflow_definition_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Workflow definition not found: {}", workflow_definition_id)))?;
        WorkflowGraph::parse(&definition.definition)
    }

    /// Computes which nodes are ready to run given the checkpoint log so far: not
    /// already started, and reachable by a satisfied edge from an already-succeeded
    /// node. `join: true` nodes additionally require every declared predecessor
    /// (not just one) to have succeeded (AND-join for parallel fan-in).
    fn compute_ready_nodes<'a>(
        graph: &'a WorkflowGraph,
        node_executions: &[WorkflowNodeExecution],
        context: &serde_json::Value,
    ) -> Result<Vec<&'a WorkflowNode>, DomainError> {
        let succeeded: HashSet<&str> = node_executions.iter()
            .filter(|ne| ne.status == NodeExecutionStatus::Succeeded)
            .map(|ne| ne.node_id.as_str())
            .collect();
        let started: HashSet<&str> = node_executions.iter().map(|ne| ne.node_id.as_str()).collect();

        let mut candidates: HashSet<&str> = HashSet::new();
        for edge in &graph.edges {
            if succeeded.contains(edge.from.as_str()) && !started.contains(edge.to.as_str()) && edge.matches(context)? {
                candidates.insert(edge.to.as_str());
            }
        }

        let mut ready = Vec::new();
        for node_id in candidates {
            let node = graph.find_node(node_id)
                .ok_or_else(|| DomainError::InternalError(format!("Edge references unknown node '{}'", node_id)))?;
            let is_ready = if node.join {
                let predecessors: HashSet<&str> = graph.edges.iter()
                    .filter(|e| e.to == node_id)
                    .map(|e| e.from.as_str())
                    .collect();
                predecessors.iter().all(|p| succeeded.contains(p))
            } else {
                true
            };
            if is_ready {
                ready.push(node);
            }
        }
        Ok(ready)
    }

    /// Runs one node to completion, including its retry policy; persists a checkpoint
    /// row per attempt. `Start`/`End` are trivial markers; `Approval` halts by
    /// recording a `WaitingApproval` row instead of calling an executor.
    async fn run_node(&self, execution_id: Uuid, tenant_id: Uuid, node: &WorkflowNode, context: serde_json::Value) -> Result<serde_json::Value, DomainError> {
        match node.node_type {
            NodeType::Start | NodeType::End => {
                let mut node_execution = WorkflowNodeExecution::new(NewWorkflowNodeExecution {
                    workflow_execution_id: execution_id,
                    node_id: node.id.clone(),
                    status: NodeExecutionStatus::Succeeded,
                    attempt: 1,
                    input: Some(context),
                });
                node_execution.completed_at = Some(Utc::now());
                self.node_execution_repository.save(&node_execution).await?;
                Ok(serde_json::json!({}))
            }
            NodeType::Approval => {
                let node_execution = WorkflowNodeExecution::new(NewWorkflowNodeExecution {
                    workflow_execution_id: execution_id,
                    node_id: node.id.clone(),
                    status: NodeExecutionStatus::WaitingApproval,
                    attempt: 1,
                    input: Some(context),
                });
                self.node_execution_repository.save(&node_execution).await?;
                info!(node_id = %node.id, "Workflow paused for human approval");
                Ok(serde_json::json!({}))
            }
            NodeType::Task => {
                let executor_name = node.executor.as_deref().unwrap_or(DEFAULT_EXECUTOR);
                let executor = self.executors.get(executor_name)
                    .ok_or_else(|| DomainError::ValidationError(format!("No executor registered for '{}'", executor_name)))?;
                let max_attempts = node.retry_policy.as_ref().map(|p| p.max_attempts).unwrap_or(1).max(1);
                let backoff_seconds = node.retry_policy.as_ref().map(|p| p.backoff_seconds).unwrap_or(0);

                let mut last_error = None;
                for attempt in 1..=max_attempts {
                    let mut node_execution = WorkflowNodeExecution::new(NewWorkflowNodeExecution {
                        workflow_execution_id: execution_id,
                        node_id: node.id.clone(),
                        status: NodeExecutionStatus::Running,
                        attempt: attempt as i32,
                        input: Some(context.clone()),
                    });
                    self.node_execution_repository.save(&node_execution).await?;

                    match executor.execute(node, tenant_id, &context).await {
                        Ok(output) => {
                            node_execution.status = NodeExecutionStatus::Succeeded;
                            node_execution.output = Some(output.clone());
                            node_execution.completed_at = Some(Utc::now());
                            node_execution.updated_at = Utc::now();
                            self.node_execution_repository.update(&node_execution).await?;
                            return Ok(output);
                        }
                        Err(e) => {
                            warn!(node_id = %node.id, attempt, error = %e, "Node execution attempt failed");
                            node_execution.status = NodeExecutionStatus::Failed;
                            node_execution.error = Some(e.to_string());
                            node_execution.completed_at = Some(Utc::now());
                            node_execution.updated_at = Utc::now();
                            self.node_execution_repository.update(&node_execution).await?;
                            if attempt < max_attempts && backoff_seconds > 0 {
                                tokio::time::sleep(Duration::from_secs(backoff_seconds)).await;
                            }
                            last_error = Some(e);
                        }
                    }
                }
                Err(last_error.unwrap_or_else(|| DomainError::InternalError(format!("Node '{}' failed with no recorded error", node.id))))
            }
        }
    }

    /// Drives the scheduling loop purely from checkpoint state (the persisted node
    /// execution rows): computes the ready set, runs it concurrently so independent
    /// branches execute in parallel, merges outputs into the execution context, and
    /// repeats until nothing more is ready — at which point the execution is
    /// Completed, Failed, or paused on a human approval. Because every decision is
    /// re-derived from the DB on each call, this same loop is what makes `start_execution`
    /// and `decide_approval` (resume-from-checkpoint) share one implementation.
    async fn continue_execution(&self, execution: &mut WorkflowExecution, graph: &WorkflowGraph) -> Result<(), DomainError> {
        for _ in 0..MAX_SCHEDULING_ROUNDS {
            let node_executions = self.node_execution_repository.find_by_execution_id(execution.id).await?;

            if node_executions.iter().any(|ne| ne.status == NodeExecutionStatus::WaitingApproval) {
                execution.status = ExecutionStatus::WaitingApproval;
                self.execution_repository.update(execution).await?;
                return Ok(());
            }

            let ready = Self::compute_ready_nodes(graph, &node_executions, &execution.context)?;
            if ready.is_empty() {
                execution.status = ExecutionStatus::Completed;
                execution.completed_at = Some(Utc::now());
                execution.updated_at = Utc::now();
                self.execution_repository.update(execution).await?;
                return Ok(());
            }

            let context_snapshot = execution.context.clone();
            let results = futures::future::join_all(
                ready.iter().map(|node| self.run_node(execution.id, execution.tenant_id, node, context_snapshot.clone())),
            ).await;

            for result in results {
                match result {
                    Ok(output) => execution.merge_context(&output),
                    Err(e) => {
                        error!(execution_id = %execution.id, error = %e, "Workflow execution failed");
                        execution.status = ExecutionStatus::Failed;
                        execution.error = Some(e.to_string());
                        execution.completed_at = Some(Utc::now());
                        execution.updated_at = Utc::now();
                        self.execution_repository.update(execution).await?;
                        return Ok(());
                    }
                }
            }
            execution.updated_at = Utc::now();
            self.execution_repository.update(execution).await?;
        }

        Err(DomainError::InternalError(format!("Workflow execution {} exceeded the maximum scheduling rounds", execution.id)))
    }
}

#[async_trait]
impl WorkflowExecutionPort for WorkflowEngineService {
    async fn start_execution(&self, command: StartWorkflowExecutionCommand) -> Result<WorkflowExecutionResponse, DomainError> {
        let graph = self.load_graph(command.workflow_definition_id).await?;
        let start_node = graph.start_node()
            .ok_or_else(|| DomainError::ValidationError("Workflow definition has no Start node".into()))?
            .clone();

        let context = match command.context {
            Some(raw) => serde_json::from_str(&raw).map_err(|e| DomainError::ValidationError(format!("Invalid context JSON: {}", e)))?,
            None => serde_json::json!({}),
        };

        let mut execution = WorkflowExecution::new(NewWorkflowExecution {
            tenant_id: command.tenant_id,
            workflow_definition_id: command.workflow_definition_id,
            context,
        });
        self.execution_repository.save(&execution).await?;
        info!(execution_id = %execution.id, "Workflow execution started");

        self.run_node(execution.id, execution.tenant_id, &start_node, execution.context.clone()).await?;
        self.continue_execution(&mut execution, &graph).await?;
        Ok(Self::to_response(&execution))
    }

    async fn find_execution_by_id(&self, id: Uuid) -> Result<WorkflowExecutionResponse, DomainError> {
        let execution = self.execution_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Workflow execution not found: {}", id)))?;
        Ok(Self::to_response(&execution))
    }

    async fn find_executions_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WorkflowExecutionResponse>, DomainError> {
        let executions = self.execution_repository.find_by_tenant(tenant_id).await?;
        Ok(executions.iter().map(Self::to_response).collect())
    }

    async fn find_node_executions(&self, execution_id: Uuid) -> Result<Vec<WorkflowNodeExecutionResponse>, DomainError> {
        let node_executions = self.node_execution_repository.find_by_execution_id(execution_id).await?;
        Ok(node_executions.iter().map(Self::to_node_response).collect())
    }

    async fn decide_approval(&self, execution_id: Uuid, node_id: &str, command: ApprovalDecisionCommand) -> Result<WorkflowExecutionResponse, DomainError> {
        let mut execution = self.execution_repository.find_by_id(execution_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Workflow execution not found: {}", execution_id)))?;
        if execution.status != ExecutionStatus::WaitingApproval {
            return Err(DomainError::ValidationError("Execution is not waiting for approval".into()));
        }

        let mut node_execution = self.node_execution_repository.find_latest_by_execution_and_node(execution_id, node_id).await?
            .filter(|ne| ne.status == NodeExecutionStatus::WaitingApproval)
            .ok_or_else(|| DomainError::NotFound(format!("No pending approval for node '{}'", node_id)))?;

        node_execution.status = if command.approved { NodeExecutionStatus::Succeeded } else { NodeExecutionStatus::Failed };
        node_execution.output = command.comment.map(|c| serde_json::json!({ "comment": c }));
        node_execution.completed_at = Some(Utc::now());
        node_execution.updated_at = Utc::now();
        self.node_execution_repository.update(&node_execution).await?;

        info!(execution_id = %execution_id, node_id = %node_id, approved = command.approved, "Approval decision recorded");

        if !command.approved {
            execution.status = ExecutionStatus::Failed;
            execution.error = Some(format!("Rejected at approval node '{}'", node_id));
            execution.completed_at = Some(Utc::now());
            execution.updated_at = Utc::now();
            self.execution_repository.update(&execution).await?;
            return Ok(Self::to_response(&execution));
        }

        execution.status = ExecutionStatus::Running;
        let graph = self.load_graph(execution.workflow_definition_id).await?;
        self.continue_execution(&mut execution, &graph).await?;
        Ok(Self::to_response(&execution))
    }
}
