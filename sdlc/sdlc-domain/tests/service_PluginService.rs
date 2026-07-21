use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;
use sdlc_domain::dto::CreatePluginCommand::CreatePluginCommand;
use sdlc_domain::dto::Plugin::{NewPlugin, Plugin};
use sdlc_domain::dto::PluginResponse::PluginResponse;
use sdlc_domain::dto::UpdatePluginCommand::UpdatePluginCommand;
use sdlc_domain::port::input::PluginPort::PluginPort;
use sdlc_domain::port::output::PluginRepositoryPort::PluginRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::PluginService::PluginService;

use std::sync::Mutex;

#[derive(Default)]
struct MockPluginRepository {
    plugins: Mutex<Vec<Plugin>>,
}

#[async_trait]
impl PluginRepositoryPort for MockPluginRepository {
    async fn save(&self, plugin: &Plugin) -> Result<(), DomainError> {
        self.plugins.lock().unwrap().push(plugin.clone());
        Ok(())
    }
    async fn update(&self, plugin: &Plugin) -> Result<(), DomainError> {
        let mut plugins = self.plugins.lock().unwrap();
        if let Some(existing) = plugins.iter_mut().find(|p| p.id == plugin.id) {
            *existing = plugin.clone();
        }
        Ok(())
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Plugin>, DomainError> {
        Ok(self.plugins.lock().unwrap().iter().find(|p| p.id == id).cloned())
    }
    async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Plugin>, DomainError> {
        Ok(self.plugins.lock().unwrap().iter()
            .filter(|p| p.tenant_id == Some(tenant_id) || p.tenant_id.is_none())
            .cloned()
            .collect())
    }
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        let mut plugins = self.plugins.lock().unwrap();
        let len_before = plugins.len();
        plugins.retain(|p| p.id != id);
        Ok(plugins.len() != len_before)
    }
}

fn sample_command(tenant_id: Option<Uuid>) -> CreatePluginCommand {
    CreatePluginCommand {
        tenant_id,
        name: "Slack Notifier".into(),
        webhook_url: "https://example.com/hooks/slack".into(),
        secret: Some("s3cr3t".into()),
    }
}

#[tokio::test]
async fn create_plugin_fails_when_webhook_url_is_empty() {
    let service = PluginService::new(Arc::new(MockPluginRepository::default()));
    let mut command = sample_command(None);
    command.webhook_url = "".into();
    let result = service.create_plugin(command).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn find_plugins_by_tenant_includes_global_plugins() {
    let service = PluginService::new(Arc::new(MockPluginRepository::default()));
    let tenant_id = Uuid::new_v4();
    service.create_plugin(sample_command(None)).await.unwrap();
    service.create_plugin(sample_command(Some(tenant_id))).await.unwrap();
    service.create_plugin(sample_command(Some(Uuid::new_v4()))).await.unwrap();

    let results = service.find_plugins_by_tenant(tenant_id).await.unwrap();
    assert_eq!(results.len(), 2);
}

#[tokio::test]
async fn delete_plugin_fails_when_not_found() {
    let service = PluginService::new(Arc::new(MockPluginRepository::default()));
    let result = service.delete_plugin(Uuid::new_v4()).await;
    assert!(matches!(result, Err(DomainError::NotFound(_))));
}
