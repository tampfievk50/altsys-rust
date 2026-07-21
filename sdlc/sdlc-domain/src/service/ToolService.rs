use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::CreateToolCommand::CreateToolCommand;
use crate::dto::Tool::{NewTool, Tool};
use crate::dto::ToolResponse::ToolResponse;
use crate::dto::UpdateToolCommand::UpdateToolCommand;
use crate::port::input::ToolPort::ToolPort;
use crate::port::output::ToolRepositoryPort::ToolRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct ToolService {
    tool_repository: Arc<dyn ToolRepositoryPort>,
}

impl ToolService {
    pub fn new(tool_repository: Arc<dyn ToolRepositoryPort>) -> Self {
        Self { tool_repository }
    }

    fn to_response(tool: &Tool) -> ToolResponse {
        ToolResponse {
            id: tool.id,
            tenant_id: tool.tenant_id,
            name: tool.name.clone(),
            tool_type: tool.tool_type.clone(),
            description: tool.description.clone(),
            config: tool.config.clone(),
            is_enabled: tool.is_enabled,
            created_at: tool.created_at,
            updated_at: tool.updated_at,
            created_by: tool.created_by,
            updated_by: tool.updated_by,
        }
    }
}

#[async_trait]
impl ToolPort for ToolService {
    async fn create_tool(&self, command: CreateToolCommand) -> Result<ToolResponse, DomainError> {
        info!(name = %command.name, tool_type = %command.tool_type, "Registering tool");
        if command.name.trim().is_empty() {
            return Err(DomainError::ValidationError("Name cannot be empty".into()));
        }
        if command.tool_type.trim().is_empty() {
            return Err(DomainError::ValidationError("Tool type cannot be empty".into()));
        }
        let tool = Tool::new(NewTool {
            tenant_id: command.tenant_id,
            name: command.name,
            tool_type: command.tool_type,
            description: command.description,
            config: command.config,
        });
        self.tool_repository.save(&tool).await?;
        info!(tool_id = %tool.id, "Tool registered");
        Ok(Self::to_response(&tool))
    }

    async fn find_tool_by_id(&self, id: Uuid) -> Result<ToolResponse, DomainError> {
        let tool = self.tool_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Tool not found: {}", id)))?;
        Ok(Self::to_response(&tool))
    }

    async fn find_tools_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<ToolResponse>, DomainError> {
        let tools = self.tool_repository.find_by_tenant_including_global(tenant_id).await?;
        Ok(tools.iter().map(Self::to_response).collect())
    }

    async fn update_tool(&self, id: Uuid, command: UpdateToolCommand) -> Result<ToolResponse, DomainError> {
        info!(tool_id = %id, "Updating tool");
        let mut tool = self.tool_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Tool not found: {}", id)))?;
        if let Some(name) = command.name {
            tool.name = name;
        }
        if let Some(description) = command.description {
            tool.description = Some(description);
        }
        if let Some(config) = command.config {
            tool.config = Some(config);
        }
        if let Some(is_enabled) = command.is_enabled {
            tool.is_enabled = is_enabled;
        }
        tool.updated_at = Utc::now();
        self.tool_repository.update(&tool).await?;
        Ok(Self::to_response(&tool))
    }

    async fn delete_tool(&self, id: Uuid) -> Result<(), DomainError> {
        info!(tool_id = %id, "Deleting tool");
        let deleted = self.tool_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(tool_id = %id, "Tool not found for deletion");
            return Err(DomainError::NotFound(format!("Tool not found: {}", id)));
        }
        Ok(())
    }
}
