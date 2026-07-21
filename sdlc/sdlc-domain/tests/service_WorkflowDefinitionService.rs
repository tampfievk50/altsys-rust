use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;
use sdlc_domain::dto::CreateWorkflowDefinitionCommand::CreateWorkflowDefinitionCommand;
use sdlc_domain::dto::UpdateWorkflowDefinitionCommand::UpdateWorkflowDefinitionCommand;
use sdlc_domain::dto::WorkflowDefinition::{NewWorkflowDefinition, WorkflowDefinition};
use sdlc_domain::dto::WorkflowDefinitionResponse::WorkflowDefinitionResponse;
use sdlc_domain::dto::WorkflowGraph::WorkflowGraph;
use sdlc_domain::port::input::WorkflowDefinitionPort::WorkflowDefinitionPort;
use sdlc_domain::port::output::WorkflowDefinitionRepositoryPort::WorkflowDefinitionRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::WorkflowDefinitionService::WorkflowDefinitionService;

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
    async fn update(&self, definition: &WorkflowDefinition) -> Result<(), DomainError> {
        let mut definitions = self.definitions.lock().unwrap();
        if let Some(existing) = definitions.iter_mut().find(|d| d.id == definition.id) {
            *existing = definition.clone();
        }
        Ok(())
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<WorkflowDefinition>, DomainError> {
        Ok(self.definitions.lock().unwrap().iter().find(|d| d.id == id).cloned())
    }
    async fn find_all_by_key_and_tenant(&self, tenant_id: Uuid, key: &str) -> Result<Vec<WorkflowDefinition>, DomainError> {
        Ok(self.definitions.lock().unwrap().iter().filter(|d| d.tenant_id == tenant_id && d.key == key).cloned().collect())
    }
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WorkflowDefinition>, DomainError> {
        Ok(self.definitions.lock().unwrap().iter().filter(|d| d.tenant_id == tenant_id).cloned().collect())
    }
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        let mut definitions = self.definitions.lock().unwrap();
        let len_before = definitions.len();
        definitions.retain(|d| d.id != id);
        Ok(definitions.len() != len_before)
    }
}

fn linear_definition_json() -> String {
    r#"{
        "nodes": [
            {"id": "start", "name": "Start", "node_type": "start"},
            {"id": "end", "name": "End", "node_type": "end"}
        ],
        "edges": [
            {"from": "start", "to": "end"}
        ]
    }"#.into()
}

fn sample_command(tenant_id: Uuid) -> CreateWorkflowDefinitionCommand {
    CreateWorkflowDefinitionCommand {
        tenant_id,
        key: "release-flow".into(),
        name: "Release Flow".into(),
        description: None,
        definition: linear_definition_json(),
    }
}

#[tokio::test]
async fn create_workflow_definition_starts_at_version_one() {
    let service = WorkflowDefinitionService::new(Arc::new(MockDefinitionRepository::default()));
    let response = service.create_workflow_definition(sample_command(Uuid::new_v4())).await.unwrap();
    assert_eq!(response.version, 1);
}

#[tokio::test]
async fn create_workflow_definition_increments_version_for_the_same_key() {
    let service = WorkflowDefinitionService::new(Arc::new(MockDefinitionRepository::default()));
    let tenant_id = Uuid::new_v4();
    service.create_workflow_definition(sample_command(tenant_id)).await.unwrap();
    let second = service.create_workflow_definition(sample_command(tenant_id)).await.unwrap();
    assert_eq!(second.version, 2);
}

#[tokio::test]
async fn create_workflow_definition_rejects_invalid_graph() {
    let service = WorkflowDefinitionService::new(Arc::new(MockDefinitionRepository::default()));
    let mut command = sample_command(Uuid::new_v4());
    command.definition = r#"{"nodes": [], "edges": []}"#.into();
    let result = service.create_workflow_definition(command).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn delete_workflow_definition_fails_when_not_found() {
    let service = WorkflowDefinitionService::new(Arc::new(MockDefinitionRepository::default()));
    let result = service.delete_workflow_definition(Uuid::new_v4()).await;
    assert!(matches!(result, Err(DomainError::NotFound(_))));
}
