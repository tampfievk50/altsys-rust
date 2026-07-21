use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tracing::info;
use uuid::Uuid;
use sdlc_domain::dto::ExecuteToolCommand::ExecuteToolCommand;
use sdlc_domain::dto::ToolExecutionContext::ToolExecutionContext;
use sdlc_domain::dto::ToolExecutionResult::ToolExecutionResult;
use sdlc_domain::port::input::ToolExecutionPort::ToolExecutionPort;
use sdlc_domain::port::output::ToolExecutorPort::ToolExecutorPort;
use sdlc_domain::port::output::ToolRepositoryPort::ToolRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::ToolExecutionService::ToolExecutionService;

use sdlc_domain::dto::Tool::{NewTool, Tool};
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
    async fn find_by_tenant_including_global(&self, _tenant_id: Uuid) -> Result<Vec<Tool>, DomainError> {
        Ok(self.tools.lock().unwrap().clone())
    }
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        let mut tools = self.tools.lock().unwrap();
        let len_before = tools.len();
        tools.retain(|t| t.id != id);
        Ok(tools.len() != len_before)
    }
}

struct EchoExecutor;

#[async_trait]
impl ToolExecutorPort for EchoExecutor {
    fn tool_type(&self) -> &'static str {
        "echo"
    }

    async fn execute(&self, context: ToolExecutionContext) -> Result<ToolExecutionResult, DomainError> {
        Ok(ToolExecutionResult::ok(context.action, 0))
    }
}

fn repo_with_tool(tool_type: &str, is_enabled: bool) -> (Arc<MockToolRepository>, Uuid) {
    let repo = Arc::new(MockToolRepository::default());
    let tool = {
        let mut t = Tool::new(NewTool {
            tenant_id: None,
            name: "sample".into(),
            tool_type: tool_type.into(),
            description: None,
            config: None,
        });
        t.is_enabled = is_enabled;
        t
    };
    let id = tool.id;
    repo.tools.lock().unwrap().push(tool);
    (repo, id)
}

#[tokio::test]
async fn execute_tool_dispatches_to_matching_executor() {
    let (repo, tool_id) = repo_with_tool("echo", true);
    let service = ToolExecutionService::new(repo, vec![Arc::new(EchoExecutor)]);
    let result = service.execute_tool(tool_id, ExecuteToolCommand {
        action: "ping".into(),
        parameters: HashMap::new(),
        working_directory: None,
    }).await.unwrap();
    assert!(result.success);
    assert_eq!(result.output, "ping");
}

#[tokio::test]
async fn execute_tool_fails_when_disabled() {
    let (repo, tool_id) = repo_with_tool("echo", false);
    let service = ToolExecutionService::new(repo, vec![Arc::new(EchoExecutor)]);
    let result = service.execute_tool(tool_id, ExecuteToolCommand {
        action: "ping".into(),
        parameters: HashMap::new(),
        working_directory: None,
    }).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn execute_tool_fails_when_no_executor_registered() {
    let (repo, tool_id) = repo_with_tool("git", true);
    let service = ToolExecutionService::new(repo, vec![Arc::new(EchoExecutor)]);
    let result = service.execute_tool(tool_id, ExecuteToolCommand {
        action: "status".into(),
        parameters: HashMap::new(),
        working_directory: None,
    }).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn execute_tool_fails_when_tool_not_found() {
    let repo = Arc::new(MockToolRepository::default());
    let service = ToolExecutionService::new(repo, vec![Arc::new(EchoExecutor)]);
    let result = service.execute_tool(Uuid::new_v4(), ExecuteToolCommand {
        action: "ping".into(),
        parameters: HashMap::new(),
        working_directory: None,
    }).await;
    assert!(matches!(result, Err(DomainError::NotFound(_))));
}
