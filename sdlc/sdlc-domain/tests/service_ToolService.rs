use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;
use sdlc_domain::dto::CreateToolCommand::CreateToolCommand;
use sdlc_domain::dto::Tool::{NewTool, Tool};
use sdlc_domain::dto::ToolResponse::ToolResponse;
use sdlc_domain::dto::UpdateToolCommand::UpdateToolCommand;
use sdlc_domain::port::input::ToolPort::ToolPort;
use sdlc_domain::port::output::ToolRepositoryPort::ToolRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::ToolService::ToolService;

use std::sync::Mutex;

#[derive(Default)]
struct MockToolRepository {
    tools: Mutex<Vec<Tool>>,
}

#[async_trait]
impl ToolRepositoryPort for MockToolRepository {
    async fn save(&self, tool: &Tool) -> Result<(), DomainError> {
        self.tools.lock().unwrap().push(tool.clone());
        Ok(())
    }

    async fn update(&self, tool: &Tool) -> Result<(), DomainError> {
        let mut tools = self.tools.lock().unwrap();
        if let Some(existing) = tools.iter_mut().find(|t| t.id == tool.id) {
            *existing = tool.clone();
        }
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tool>, DomainError> {
        Ok(self.tools.lock().unwrap().iter().find(|t| t.id == id).cloned())
    }

    async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Tool>, DomainError> {
        Ok(self.tools.lock().unwrap().iter()
            .filter(|t| t.tenant_id == Some(tenant_id) || t.tenant_id.is_none())
            .cloned()
            .collect())
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        let mut tools = self.tools.lock().unwrap();
        let len_before = tools.len();
        tools.retain(|t| t.id != id);
        Ok(tools.len() != len_before)
    }
}

fn sample_command(tenant_id: Option<Uuid>) -> CreateToolCommand {
    CreateToolCommand {
        tenant_id,
        name: "primary-git".into(),
        tool_type: "git".into(),
        description: None,
        config: None,
    }
}

#[tokio::test]
async fn create_tool_fails_when_name_is_empty() {
    let service = ToolService::new(Arc::new(MockToolRepository::default()));
    let mut command = sample_command(None);
    command.name = "".into();
    let result = service.create_tool(command).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn find_tools_by_tenant_includes_global_tools() {
    let service = ToolService::new(Arc::new(MockToolRepository::default()));
    let tenant_id = Uuid::new_v4();
    service.create_tool(sample_command(None)).await.unwrap();
    service.create_tool(sample_command(Some(tenant_id))).await.unwrap();
    service.create_tool(sample_command(Some(Uuid::new_v4()))).await.unwrap();

    let results = service.find_tools_by_tenant(tenant_id).await.unwrap();
    assert_eq!(results.len(), 2);
}

#[tokio::test]
async fn update_tool_applies_partial_changes() {
    let service = ToolService::new(Arc::new(MockToolRepository::default()));
    let created = service.create_tool(sample_command(None)).await.unwrap();
    let updated = service.update_tool(created.id, UpdateToolCommand {
        name: None,
        description: Some("Company monorepo".into()),
        config: None,
        is_enabled: Some(false),
    }).await.unwrap();
    assert_eq!(updated.description, Some("Company monorepo".into()));
    assert!(!updated.is_enabled);
}

#[tokio::test]
async fn delete_tool_fails_when_not_found() {
    let service = ToolService::new(Arc::new(MockToolRepository::default()));
    let result = service.delete_tool(Uuid::new_v4()).await;
    assert!(matches!(result, Err(DomainError::NotFound(_))));
}
