use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::CreatePromptCommand::CreatePromptCommand;
use crate::dto::Prompt::{NewPrompt, Prompt};
use crate::dto::PromptResponse::PromptResponse;
use crate::dto::UpdatePromptCommand::UpdatePromptCommand;
use crate::port::input::PromptPort::PromptPort;
use crate::port::output::PromptRepositoryPort::PromptRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct PromptService {
    prompt_repository: Arc<dyn PromptRepositoryPort>,
}

impl PromptService {
    pub fn new(prompt_repository: Arc<dyn PromptRepositoryPort>) -> Self {
        Self { prompt_repository }
    }

    fn to_response(prompt: &Prompt) -> PromptResponse {
        PromptResponse {
            id: prompt.id,
            tenant_id: prompt.tenant_id,
            key: prompt.key.clone(),
            version: prompt.version,
            content: prompt.content.clone(),
            variables: prompt.variables.clone(),
            description: prompt.description.clone(),
            is_active: prompt.is_active,
            created_at: prompt.created_at,
            updated_at: prompt.updated_at,
            created_by: prompt.created_by,
            updated_by: prompt.updated_by,
        }
    }

    fn latest_per_key(prompts: Vec<Prompt>) -> Vec<Prompt> {
        let mut latest: HashMap<String, Prompt> = HashMap::new();
        for prompt in prompts {
            latest
                .entry(prompt.key.clone())
                .and_modify(|existing| {
                    if prompt.version > existing.version {
                        *existing = prompt.clone();
                    }
                })
                .or_insert(prompt);
        }
        latest.into_values().collect()
    }
}

#[async_trait]
impl PromptPort for PromptService {
    async fn create_prompt(&self, command: CreatePromptCommand) -> Result<PromptResponse, DomainError> {
        info!(key = %command.key, "Creating prompt version");
        if command.key.trim().is_empty() {
            return Err(DomainError::ValidationError("Prompt key cannot be empty".into()));
        }
        if command.content.trim().is_empty() {
            return Err(DomainError::ValidationError("Prompt content cannot be empty".into()));
        }
        let existing_versions = self.prompt_repository.find_all_by_key_and_tenant(command.tenant_id, &command.key).await?;
        let next_version = existing_versions.iter().map(|p| p.version).max().unwrap_or(0) + 1;
        let prompt = Prompt::new(NewPrompt {
            tenant_id: command.tenant_id,
            key: command.key,
            version: next_version,
            content: command.content,
            variables: command.variables,
            description: command.description,
        });
        self.prompt_repository.save(&prompt).await?;
        info!(prompt_id = %prompt.id, version = prompt.version, "Prompt version created");
        Ok(Self::to_response(&prompt))
    }

    async fn find_prompt_by_id(&self, id: Uuid) -> Result<PromptResponse, DomainError> {
        let prompt = self.prompt_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Prompt not found: {}", id)))?;
        Ok(Self::to_response(&prompt))
    }

    async fn find_latest_prompt_by_key(&self, tenant_id: Uuid, key: &str) -> Result<PromptResponse, DomainError> {
        let versions = self.prompt_repository.find_all_by_key_and_tenant(tenant_id, key).await?;
        let latest = versions.into_iter().max_by_key(|p| p.version)
            .ok_or_else(|| DomainError::NotFound(format!("Prompt not found for key: {}", key)))?;
        Ok(Self::to_response(&latest))
    }

    async fn find_prompt_versions_by_key(&self, tenant_id: Uuid, key: &str) -> Result<Vec<PromptResponse>, DomainError> {
        let mut versions = self.prompt_repository.find_all_by_key_and_tenant(tenant_id, key).await?;
        versions.sort_by_key(|p| p.version);
        Ok(versions.iter().map(Self::to_response).collect())
    }

    async fn find_prompts_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<PromptResponse>, DomainError> {
        let prompts = self.prompt_repository.find_by_tenant(tenant_id).await?;
        Ok(Self::latest_per_key(prompts).iter().map(Self::to_response).collect())
    }

    async fn update_prompt(&self, id: Uuid, command: UpdatePromptCommand) -> Result<PromptResponse, DomainError> {
        info!(prompt_id = %id, "Updating prompt");
        let mut prompt = self.prompt_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Prompt not found: {}", id)))?;
        if let Some(content) = command.content {
            if content.trim().is_empty() {
                return Err(DomainError::ValidationError("Prompt content cannot be empty".into()));
            }
            prompt.content = content;
        }
        if let Some(variables) = command.variables {
            prompt.variables = Some(variables);
        }
        if let Some(description) = command.description {
            prompt.description = Some(description);
        }
        if let Some(is_active) = command.is_active {
            prompt.is_active = is_active;
        }
        prompt.updated_at = Utc::now();
        self.prompt_repository.update(&prompt).await?;
        Ok(Self::to_response(&prompt))
    }

    async fn delete_prompt(&self, id: Uuid) -> Result<(), DomainError> {
        info!(prompt_id = %id, "Deleting prompt");
        let deleted = self.prompt_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(prompt_id = %id, "Prompt not found for deletion");
            return Err(DomainError::NotFound(format!("Prompt not found: {}", id)));
        }
        Ok(())
    }
}
