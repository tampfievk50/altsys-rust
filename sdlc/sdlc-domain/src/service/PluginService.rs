use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::CreatePluginCommand::CreatePluginCommand;
use crate::dto::Plugin::{NewPlugin, Plugin};
use crate::dto::PluginResponse::PluginResponse;
use crate::dto::UpdatePluginCommand::UpdatePluginCommand;
use crate::port::input::PluginPort::PluginPort;
use crate::port::output::PluginRepositoryPort::PluginRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct PluginService {
    plugin_repository: Arc<dyn PluginRepositoryPort>,
}

impl PluginService {
    pub fn new(plugin_repository: Arc<dyn PluginRepositoryPort>) -> Self {
        Self { plugin_repository }
    }

    fn to_response(plugin: &Plugin) -> PluginResponse {
        PluginResponse {
            id: plugin.id,
            tenant_id: plugin.tenant_id,
            name: plugin.name.clone(),
            webhook_url: plugin.webhook_url.clone(),
            is_active: plugin.is_active,
            created_at: plugin.created_at,
            updated_at: plugin.updated_at,
            created_by: plugin.created_by,
            updated_by: plugin.updated_by,
        }
    }
}

#[async_trait]
impl PluginPort for PluginService {
    async fn create_plugin(&self, command: CreatePluginCommand) -> Result<PluginResponse, DomainError> {
        info!(name = %command.name, "Registering plugin");
        if command.name.trim().is_empty() {
            return Err(DomainError::ValidationError("Name cannot be empty".into()));
        }
        if command.webhook_url.trim().is_empty() {
            return Err(DomainError::ValidationError("Webhook URL cannot be empty".into()));
        }
        let plugin = Plugin::new(NewPlugin {
            tenant_id: command.tenant_id,
            name: command.name,
            webhook_url: command.webhook_url,
            secret: command.secret,
        });
        self.plugin_repository.save(&plugin).await?;
        info!(plugin_id = %plugin.id, "Plugin registered");
        Ok(Self::to_response(&plugin))
    }

    async fn find_plugin_by_id(&self, id: Uuid) -> Result<PluginResponse, DomainError> {
        let plugin = self.plugin_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Plugin not found: {}", id)))?;
        Ok(Self::to_response(&plugin))
    }

    async fn find_plugins_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<PluginResponse>, DomainError> {
        let plugins = self.plugin_repository.find_by_tenant_including_global(tenant_id).await?;
        Ok(plugins.iter().map(Self::to_response).collect())
    }

    async fn update_plugin(&self, id: Uuid, command: UpdatePluginCommand) -> Result<PluginResponse, DomainError> {
        info!(plugin_id = %id, "Updating plugin");
        let mut plugin = self.plugin_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Plugin not found: {}", id)))?;
        if let Some(name) = command.name {
            plugin.name = name;
        }
        if let Some(webhook_url) = command.webhook_url {
            plugin.webhook_url = webhook_url;
        }
        if let Some(secret) = command.secret {
            plugin.secret = Some(secret);
        }
        if let Some(is_active) = command.is_active {
            plugin.is_active = is_active;
        }
        plugin.updated_at = Utc::now();
        self.plugin_repository.update(&plugin).await?;
        Ok(Self::to_response(&plugin))
    }

    async fn delete_plugin(&self, id: Uuid) -> Result<(), DomainError> {
        info!(plugin_id = %id, "Deleting plugin");
        let deleted = self.plugin_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(plugin_id = %id, "Plugin not found for deletion");
            return Err(DomainError::NotFound(format!("Plugin not found: {}", id)));
        }
        Ok(())
    }
}
