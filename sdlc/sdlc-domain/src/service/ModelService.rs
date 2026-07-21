use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::CreateModelCommand::CreateModelCommand;
use crate::dto::Model::{Model, NewModel};
use crate::dto::ModelResponse::ModelResponse;
use crate::dto::UpdateModelCommand::UpdateModelCommand;
use crate::port::input::ModelPort::ModelPort;
use crate::port::output::ModelRepositoryPort::ModelRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct ModelService {
    model_repository: Arc<dyn ModelRepositoryPort>,
}

impl ModelService {
    pub fn new(model_repository: Arc<dyn ModelRepositoryPort>) -> Self {
        Self { model_repository }
    }

    fn to_response(model: &Model) -> ModelResponse {
        ModelResponse {
            id: model.id,
            tenant_id: model.tenant_id,
            provider: model.provider.clone(),
            model_name: model.model_name.clone(),
            capability: model.capability.clone(),
            credential_id: model.credential_id,
            endpoint_url: model.endpoint_url.clone(),
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
        }
    }
}

#[async_trait]
impl ModelPort for ModelService {
    async fn create_model(&self, command: CreateModelCommand) -> Result<ModelResponse, DomainError> {
        info!(provider = %command.provider, model_name = %command.model_name, "Registering model");
        if command.provider.trim().is_empty() {
            return Err(DomainError::ValidationError("Provider cannot be empty".into()));
        }
        if command.model_name.trim().is_empty() {
            return Err(DomainError::ValidationError("Model name cannot be empty".into()));
        }
        if command.capability.trim().is_empty() {
            return Err(DomainError::ValidationError("Capability cannot be empty".into()));
        }
        let model = Model::new(NewModel {
            tenant_id: command.tenant_id,
            provider: command.provider,
            model_name: command.model_name,
            capability: command.capability,
            credential_id: command.credential_id,
            endpoint_url: command.endpoint_url,
        });
        self.model_repository.save(&model).await?;
        info!(model_id = %model.id, "Model registered");
        Ok(Self::to_response(&model))
    }

    async fn find_model_by_id(&self, id: Uuid) -> Result<ModelResponse, DomainError> {
        let model = self.model_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Model not found: {}", id)))?;
        Ok(Self::to_response(&model))
    }

    async fn find_models_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<ModelResponse>, DomainError> {
        let models = self.model_repository.find_by_tenant_including_global(tenant_id).await?;
        Ok(models.iter().map(Self::to_response).collect())
    }

    async fn update_model(&self, id: Uuid, command: UpdateModelCommand) -> Result<ModelResponse, DomainError> {
        info!(model_id = %id, "Updating model");
        let mut model = self.model_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Model not found: {}", id)))?;
        if let Some(provider) = command.provider {
            model.provider = provider;
        }
        if let Some(model_name) = command.model_name {
            model.model_name = model_name;
        }
        if let Some(capability) = command.capability {
            model.capability = capability;
        }
        if let Some(credential_id) = command.credential_id {
            model.credential_id = Some(credential_id);
        }
        if let Some(endpoint_url) = command.endpoint_url {
            model.endpoint_url = Some(endpoint_url);
        }
        if let Some(is_active) = command.is_active {
            model.is_active = is_active;
        }
        model.updated_at = Utc::now();
        self.model_repository.update(&model).await?;
        Ok(Self::to_response(&model))
    }

    async fn delete_model(&self, id: Uuid) -> Result<(), DomainError> {
        info!(model_id = %id, "Deleting model");
        let deleted = self.model_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(model_id = %id, "Model not found for deletion");
            return Err(DomainError::NotFound(format!("Model not found: {}", id)));
        }
        Ok(())
    }
}
