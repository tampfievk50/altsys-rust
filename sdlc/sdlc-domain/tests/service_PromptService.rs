use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;
use sdlc_domain::dto::CreatePromptCommand::CreatePromptCommand;
use sdlc_domain::dto::Prompt::{NewPrompt, Prompt};
use sdlc_domain::dto::PromptResponse::PromptResponse;
use sdlc_domain::dto::UpdatePromptCommand::UpdatePromptCommand;
use sdlc_domain::port::input::PromptPort::PromptPort;
use sdlc_domain::port::output::PromptRepositoryPort::PromptRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::PromptService::PromptService;

use std::sync::Mutex;

#[derive(Default)]
struct MockPromptRepository {
    prompts: Mutex<Vec<Prompt>>,
}

#[async_trait]
impl PromptRepositoryPort for MockPromptRepository {
    async fn save(&self, prompt: &Prompt) -> Result<(), DomainError> {
        self.prompts.lock().unwrap().push(prompt.clone());
        Ok(())
    }

    async fn update(&self, prompt: &Prompt) -> Result<(), DomainError> {
        let mut prompts = self.prompts.lock().unwrap();
        if let Some(existing) = prompts.iter_mut().find(|p| p.id == prompt.id) {
            *existing = prompt.clone();
        }
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Prompt>, DomainError> {
        Ok(self.prompts.lock().unwrap().iter().find(|p| p.id == id).cloned())
    }

    async fn find_all_by_key_and_tenant(&self, tenant_id: Uuid, key: &str) -> Result<Vec<Prompt>, DomainError> {
        Ok(self.prompts.lock().unwrap().iter().filter(|p| p.tenant_id == tenant_id && p.key == key).cloned().collect())
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Prompt>, DomainError> {
        Ok(self.prompts.lock().unwrap().iter().filter(|p| p.tenant_id == tenant_id).cloned().collect())
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        let mut prompts = self.prompts.lock().unwrap();
        let len_before = prompts.len();
        prompts.retain(|p| p.id != id);
        Ok(prompts.len() != len_before)
    }
}

fn sample_command(tenant_id: Uuid) -> CreatePromptCommand {
    CreatePromptCommand {
        tenant_id,
        key: "planner.generate_plan".into(),
        content: "Generate an implementation plan for {{ticket}}".into(),
        variables: Some("[\"ticket\"]".into()),
        description: None,
    }
}

#[tokio::test]
async fn create_prompt_starts_at_version_one() {
    let service = PromptService::new(Arc::new(MockPromptRepository::default()));
    let response = service.create_prompt(sample_command(Uuid::new_v4())).await.unwrap();
    assert_eq!(response.version, 1);
}

#[tokio::test]
async fn create_prompt_increments_version_for_the_same_key() {
    let service = PromptService::new(Arc::new(MockPromptRepository::default()));
    let tenant_id = Uuid::new_v4();
    service.create_prompt(sample_command(tenant_id)).await.unwrap();
    let second = service.create_prompt(sample_command(tenant_id)).await.unwrap();
    assert_eq!(second.version, 2);
}

#[tokio::test]
async fn create_prompt_fails_when_content_is_empty() {
    let service = PromptService::new(Arc::new(MockPromptRepository::default()));
    let mut command = sample_command(Uuid::new_v4());
    command.content = "".into();
    let result = service.create_prompt(command).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn find_latest_prompt_by_key_returns_highest_version() {
    let service = PromptService::new(Arc::new(MockPromptRepository::default()));
    let tenant_id = Uuid::new_v4();
    service.create_prompt(sample_command(tenant_id)).await.unwrap();
    service.create_prompt(sample_command(tenant_id)).await.unwrap();
    let latest = service.find_latest_prompt_by_key(tenant_id, "planner.generate_plan").await.unwrap();
    assert_eq!(latest.version, 2);
}

#[tokio::test]
async fn find_prompts_by_tenant_returns_latest_per_key_only() {
    let service = PromptService::new(Arc::new(MockPromptRepository::default()));
    let tenant_id = Uuid::new_v4();
    service.create_prompt(sample_command(tenant_id)).await.unwrap();
    service.create_prompt(sample_command(tenant_id)).await.unwrap();
    let mut other_key_command = sample_command(tenant_id);
    other_key_command.key = "reviewer.review_code".into();
    service.create_prompt(other_key_command).await.unwrap();

    let results = service.find_prompts_by_tenant(tenant_id).await.unwrap();
    assert_eq!(results.len(), 2);
    let planner = results.iter().find(|p| p.key == "planner.generate_plan").unwrap();
    assert_eq!(planner.version, 2);
}

#[tokio::test]
async fn delete_prompt_fails_when_not_found() {
    let service = PromptService::new(Arc::new(MockPromptRepository::default()));
    let result = service.delete_prompt(Uuid::new_v4()).await;
    assert!(matches!(result, Err(DomainError::NotFound(_))));
}
