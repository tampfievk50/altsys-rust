use std::collections::HashMap;
use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::ExecuteToolCommand::ExecuteToolCommand;
use sdlc_domain::dto::TaskOverrideResponse::TaskOverrideResponse;
use sdlc_domain::r#enum::DomainError::DomainError;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::TaskOverridePayloads::UpdateTaskSummaryRequest;

#[utoipa::path(
    get,
    path = "/api/v1/projects/{project_id}/task-overrides",
    tag = "Tasks",
    params(("project_id" = Uuid, Path, description = "Project ID")),
    responses(
        (status = 200, description = "User-edited summaries for this project's tasks", body = ApiResponse<Vec<TaskOverrideResponse>>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_task_overrides(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<TaskOverrideResponse>>>, AppError> {
    let overrides = state.task_override_service.find_overrides_by_project(project_id).await?;
    Ok(Json(ApiResponse::success(overrides)))
}

/// Writes the new summary to the real Jira ticket first (Jira stays the
/// source of truth), then records the same value as a local override so the
/// Tasks page reflects it immediately instead of waiting for the next
/// ingested event to confirm it.
#[utoipa::path(
    put,
    path = "/api/v1/projects/{project_id}/tickets/{ticket_key}",
    tag = "Tasks",
    params(
        ("project_id" = Uuid, Path, description = "Project ID"),
        ("ticket_key" = String, Path, description = "Jira ticket key, e.g. SCRUM-1")
    ),
    request_body = UpdateTaskSummaryRequest,
    responses(
        (status = 200, description = "Summary updated in Jira and recorded locally", body = ApiResponse<TaskOverrideResponse>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn update_task_summary(
    State(state): State<Arc<AppState>>,
    Path((project_id, ticket_key)): Path<(Uuid, String)>,
    Json(payload): Json<UpdateTaskSummaryRequest>,
) -> Result<Json<ApiResponse<TaskOverrideResponse>>, AppError> {
    let project = state.project_service.find_project_by_id(project_id).await?;
    let jira_tool_id = project.jira_tool_id
        .ok_or_else(|| AppError(DomainError::ValidationError("Project has no Jira config linked".into())))?;
    let tool = state.tool_service.find_tool_by_id(jira_tool_id).await?;

    let config: serde_json::Value = tool.config.as_deref()
        .and_then(|c| serde_json::from_str(c).ok())
        .unwrap_or_default();
    let email = config.get("email").and_then(|v| v.as_str())
        .ok_or_else(|| AppError(DomainError::ValidationError("Jira tool config missing 'email'".into())))?
        .to_string();
    let credential_id: Uuid = config.get("credential_id").and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| AppError(DomainError::ValidationError("Jira tool config missing 'credential_id'".into())))?;
    let api_token = state.credential_service.reveal_credential_secret(credential_id).await?.secret;

    let mut parameters = HashMap::new();
    parameters.insert("issue_key".to_string(), ticket_key.clone());
    parameters.insert("summary".to_string(), payload.summary.clone());
    parameters.insert("email".to_string(), email);
    parameters.insert("api_token".to_string(), api_token);

    let result = state.tool_execution_service.execute_tool(jira_tool_id, ExecuteToolCommand {
        action: "update_issue".to_string(),
        parameters,
        working_directory: None,
    }).await?;

    if !result.success {
        return Err(AppError(DomainError::InternalError(result.error.unwrap_or_else(|| "Jira update failed".into()))));
    }

    let updated = state.task_override_service.set_summary_override(project_id, ticket_key, payload.summary).await?;
    Ok(Json(ApiResponse::success(updated)))
}
