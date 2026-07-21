use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;
use sdlc_domain::dto::CreateModelCommand::CreateModelCommand;
use sdlc_domain::dto::Model::{Model, NewModel};
use sdlc_domain::dto::ModelResponse::ModelResponse;
use sdlc_domain::dto::UpdateModelCommand::UpdateModelCommand;
use sdlc_domain::port::input::ModelPort::ModelPort;
use sdlc_domain::port::output::ModelRepositoryPort::ModelRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::ModelService::ModelService;

use std::sync::Mutex;

#[derive(Default)]
struct MockModelRepository {
    models: Mutex<Vec<Model>>,
}

#[async_trait]
impl ModelRepositoryPort for MockModelRepository {
    async fn save(&self, model: &Model) -> Result<(), DomainError> {
        self.models.lock().unwrap().push(model.clone());
        Ok(())
    }

    async fn update(&self, model: &Model) -> Result<(), DomainError> {
        let mut models = self.models.lock().unwrap();
        if let Some(existing) = models.iter_mut().find(|m| m.id == model.id) {
            *existing = model.clone();
        }
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, DomainError> {
        Ok(self.models.lock().unwrap().iter().find(|m| m.id == id).cloned())
    }

    async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Model>, DomainError> {
        Ok(self.models.lock().unwrap().iter()
            .filter(|m| m.tenant_id == Some(tenant_id) || m.tenant_id.is_none())
            .cloned()
            .collect())
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        let mut models = self.models.lock().unwrap();
        let len_before = models.len();
        models.retain(|m| m.id != id);
        Ok(models.len() != len_before)
    }
}

fn sample_command(tenant_id: Option<Uuid>) -> CreateModelCommand {
    CreateModelCommand {
        tenant_id,
        provider: "anthropic".into(),
        model_name: "claude-sonnet-5".into(),
        capability: "coding".into(),
        credential_id: None,
        endpoint_url: None,
    }
}

#[tokio::test]
async fn create_model_fails_when_provider_is_empty() {
    let service = ModelService::new(Arc::new(MockModelRepository::default()));
    let mut command = sample_command(None);
    command.provider = "".into();
    let result = service.create_model(command).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn find_models_by_tenant_includes_global_models() {
    let service = ModelService::new(Arc::new(MockModelRepository::default()));
    let tenant_id = Uuid::new_v4();
    service.create_model(sample_command(None)).await.unwrap();
    service.create_model(sample_command(Some(tenant_id))).await.unwrap();
    service.create_model(sample_command(Some(Uuid::new_v4()))).await.unwrap();

    let results = service.find_models_by_tenant(tenant_id).await.unwrap();
    assert_eq!(results.len(), 2);
}

#[tokio::test]
async fn find_model_by_id_returns_not_found_for_unknown_id() {
    let service = ModelService::new(Arc::new(MockModelRepository::default()));
    let result = service.find_model_by_id(Uuid::new_v4()).await;
    assert!(matches!(result, Err(DomainError::NotFound(_))));
}

#[tokio::test]
async fn update_model_applies_partial_changes() {
    let service = ModelService::new(Arc::new(MockModelRepository::default()));
    let created = service.create_model(sample_command(None)).await.unwrap();
    let updated = service.update_model(created.id, UpdateModelCommand {
        provider: None,
        model_name: None,
        capability: Some("planning".into()),
        credential_id: None,
        endpoint_url: None,
        is_active: Some(false),
    }).await.unwrap();
    assert_eq!(updated.capability, "planning");
    assert!(!updated.is_active);
}

#[tokio::test]
async fn delete_model_fails_when_not_found() {
    let service = ModelService::new(Arc::new(MockModelRepository::default()));
    let result = service.delete_model(Uuid::new_v4()).await;
    assert!(matches!(result, Err(DomainError::NotFound(_))));
}
