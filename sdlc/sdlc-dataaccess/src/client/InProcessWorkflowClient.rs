use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use sdlc_domain::dto::StartWorkflowExecutionCommand::StartWorkflowExecutionCommand;
use sdlc_domain::port::input::WorkflowDefinitionPort::WorkflowDefinitionPort;
use sdlc_domain::port::input::WorkflowExecutionPort::WorkflowExecutionPort;
use sdlc_domain::port::output::WorkflowClientPort::WorkflowClientPort;
use sdlc_domain::r#enum::DomainError::DomainError;

/// Replaces the old HTTP call to the Workflow service (automation engine
/// side) with a direct call to the merged `WorkflowEngineService`.
pub struct InProcessWorkflowClient {
    workflow_execution_port: Arc<dyn WorkflowExecutionPort>,
    workflow_definition_port: Arc<dyn WorkflowDefinitionPort>,
}

impl InProcessWorkflowClient {
    pub fn new(workflow_execution_port: Arc<dyn WorkflowExecutionPort>, workflow_definition_port: Arc<dyn WorkflowDefinitionPort>) -> Self {
        Self { workflow_execution_port, workflow_definition_port }
    }
}

#[async_trait]
impl WorkflowClientPort for InProcessWorkflowClient {
    async fn start_execution(&self, workflow_definition_id: Uuid, tenant_id: Uuid, context: serde_json::Value) -> Result<serde_json::Value, DomainError> {
        let execution = self.workflow_execution_port
            .start_execution(StartWorkflowExecutionCommand { tenant_id, workflow_definition_id, context: Some(context.to_string()) })
            .await?;
        serde_json::to_value(&execution).map_err(|e| DomainError::InternalError(format!("Failed to serialize workflow execution: {}", e)))
    }

    async fn find_definition_id_by_key(&self, tenant_id: Uuid, key: &str) -> Result<Uuid, DomainError> {
        let definition = self.workflow_definition_port.find_latest_workflow_definition_by_key(tenant_id, key).await?;
        Ok(definition.id)
    }
}
