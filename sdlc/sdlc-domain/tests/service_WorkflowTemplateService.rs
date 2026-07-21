use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;
use sdlc_domain::dto::CreateWorkflowTemplateCommand::CreateWorkflowTemplateCommand;
use sdlc_domain::dto::InstantiateTemplateCommand::InstantiateTemplateCommand;
use sdlc_domain::dto::InstantiateTemplateResponse::InstantiateTemplateResponse;
use sdlc_domain::dto::UpdateWorkflowTemplateCommand::UpdateWorkflowTemplateCommand;
use sdlc_domain::dto::WorkflowTemplate::{NewWorkflowTemplate, WorkflowTemplate};
use sdlc_domain::dto::WorkflowTemplateResponse::WorkflowTemplateResponse;
use sdlc_domain::port::input::WorkflowTemplatePort::WorkflowTemplatePort;
use sdlc_domain::port::output::WorkflowTemplateRepositoryPort::WorkflowTemplateRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::WorkflowTemplateService::WorkflowTemplateService;

use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Default)]
struct MockTemplateRepository {
    templates: Mutex<Vec<WorkflowTemplate>>,
}

#[async_trait]
impl WorkflowTemplateRepositoryPort for MockTemplateRepository {
    async fn save(&self, template: &WorkflowTemplate) -> Result<(), DomainError> {
        self.templates.lock().unwrap().push(template.clone());
        Ok(())
    }
    async fn update(&self, template: &WorkflowTemplate) -> Result<(), DomainError> {
        let mut templates = self.templates.lock().unwrap();
        if let Some(existing) = templates.iter_mut().find(|t| t.id == template.id) {
            *existing = template.clone();
        }
        Ok(())
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<WorkflowTemplate>, DomainError> {
        Ok(self.templates.lock().unwrap().iter().find(|t| t.id == id).cloned())
    }
    async fn find_all_by_key_and_tenant(&self, tenant_id: Uuid, key: &str) -> Result<Vec<WorkflowTemplate>, DomainError> {
        Ok(self.templates.lock().unwrap().iter().filter(|t| t.tenant_id == tenant_id && t.key == key).cloned().collect())
    }
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WorkflowTemplate>, DomainError> {
        Ok(self.templates.lock().unwrap().iter().filter(|t| t.tenant_id == tenant_id).cloned().collect())
    }
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        let mut templates = self.templates.lock().unwrap();
        let len_before = templates.len();
        templates.retain(|t| t.id != id);
        Ok(templates.len() != len_before)
    }
}

fn sample_command(tenant_id: Uuid) -> CreateWorkflowTemplateCommand {
    CreateWorkflowTemplateCommand {
        tenant_id,
        key: "bug-fix".into(),
        name: "Bug Fix Template".into(),
        description: None,
        definition_template: r#"{"nodes":[{"id":"start","name":"Start","node_type":"start","executor":"{{executor}}"}]}"#.into(),
    }
}

#[tokio::test]
async fn create_template_starts_at_version_one() {
    let service = WorkflowTemplateService::new(Arc::new(MockTemplateRepository::default()));
    let response = service.create_template(sample_command(Uuid::new_v4())).await.unwrap();
    assert_eq!(response.version, 1);
}

#[tokio::test]
async fn create_template_increments_version_for_the_same_key() {
    let service = WorkflowTemplateService::new(Arc::new(MockTemplateRepository::default()));
    let tenant_id = Uuid::new_v4();
    service.create_template(sample_command(tenant_id)).await.unwrap();
    let second = service.create_template(sample_command(tenant_id)).await.unwrap();
    assert_eq!(second.version, 2);
}

#[tokio::test]
async fn instantiate_template_substitutes_placeholders() {
    let service = WorkflowTemplateService::new(Arc::new(MockTemplateRepository::default()));
    let created = service.create_template(sample_command(Uuid::new_v4())).await.unwrap();

    let mut parameters = HashMap::new();
    parameters.insert("executor".to_string(), "noop".to_string());
    let response = service.instantiate_template(created.id, InstantiateTemplateCommand { parameters }).await.unwrap();

    assert!(response.definition.contains(r#""executor":"noop""#));
    assert!(!response.definition.contains("{{executor}}"));
}

#[tokio::test]
async fn instantiate_template_leaves_missing_placeholders_untouched() {
    let service = WorkflowTemplateService::new(Arc::new(MockTemplateRepository::default()));
    let created = service.create_template(sample_command(Uuid::new_v4())).await.unwrap();

    let response = service.instantiate_template(created.id, InstantiateTemplateCommand { parameters: HashMap::new() }).await.unwrap();
    assert!(response.definition.contains("{{executor}}"));
}

#[tokio::test]
async fn delete_template_fails_when_not_found() {
    let service = WorkflowTemplateService::new(Arc::new(MockTemplateRepository::default()));
    let result = service.delete_template(Uuid::new_v4()).await;
    assert!(matches!(result, Err(DomainError::NotFound(_))));
}
