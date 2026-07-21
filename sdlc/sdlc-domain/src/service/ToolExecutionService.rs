use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;
use uuid::Uuid;

use crate::dto::ExecuteToolCommand::ExecuteToolCommand;
use crate::dto::ToolExecutionContext::ToolExecutionContext;
use crate::dto::ToolExecutionResult::ToolExecutionResult;
use crate::port::input::ToolExecutionPort::ToolExecutionPort;
use crate::port::output::ToolExecutorPort::ToolExecutorPort;
use crate::port::output::ToolRepositoryPort::ToolRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct ToolExecutionService {
    tool_repository: Arc<dyn ToolRepositoryPort>,
    executors: HashMap<String, Arc<dyn ToolExecutorPort>>,
}

impl ToolExecutionService {
    pub fn new(tool_repository: Arc<dyn ToolRepositoryPort>, executors: Vec<Arc<dyn ToolExecutorPort>>) -> Self {
        let executors = executors.into_iter().map(|e| (e.tool_type().to_string(), e)).collect();
        Self { tool_repository, executors }
    }
}

#[async_trait]
impl ToolExecutionPort for ToolExecutionService {
    async fn execute_tool(&self, tool_id: Uuid, command: ExecuteToolCommand) -> Result<ToolExecutionResult, DomainError> {
        if command.action.trim().is_empty() {
            return Err(DomainError::ValidationError("Action cannot be empty".into()));
        }
        let tool = self.tool_repository.find_by_id(tool_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Tool not found: {}", tool_id)))?;
        if !tool.is_enabled {
            return Err(DomainError::ValidationError(format!("Tool '{}' is disabled", tool.name)));
        }
        let executor = self.executors.get(&tool.tool_type)
            .ok_or_else(|| DomainError::ValidationError(format!("No executor registered for tool type '{}'", tool.tool_type)))?;

        info!(tool_id = %tool.id, tool_type = %tool.tool_type, action = %command.action, "Executing tool");
        executor.execute(ToolExecutionContext {
            config: tool.config.clone(),
            action: command.action,
            parameters: command.parameters,
            working_directory: command.working_directory,
        }).await
    }
}
