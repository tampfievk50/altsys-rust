use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::IngestEventCommand::IngestEventCommand;
use sdlc_domain::dto::IngestEventResponse::IngestEventResponse;
use sdlc_domain::r#enum::DomainError::DomainError;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::JiraWebhookPayloads::JiraWebhookPayload;

/// Receives Jira's outbound webhook (configured per-project via Jira Automation
/// "send web request") and turns it into an `IngestedEvent`, letting the existing
/// event/automation-rule pipeline take it from there. Deliberately outside the JWT
/// `require_auth` layer — Jira can't mint our tokens — so the path-embedded secret
/// (checked against the linked Jira Tool's `config.webhook_secret`) is the only guard.
#[utoipa::path(
    post,
    path = "/api/v1/webhooks/jira/{project_id}/{secret}",
    tag = "Jira Webhook",
    params(
        ("project_id" = Uuid, Path, description = "Project ID"),
        ("secret" = String, Path, description = "Shared secret from the linked Jira Tool's config.webhook_secret")
    ),
    request_body = JiraWebhookPayload,
    responses(
        (status = 201, description = "Event ingested and evaluated against every active automation rule", body = ApiResponse<IngestEventResponse>)
    )
)]
pub async fn receive_jira_webhook(
    State(state): State<Arc<AppState>>,
    Path((project_id, secret)): Path<(Uuid, String)>,
    Json(payload): Json<JiraWebhookPayload>,
) -> Result<Json<ApiResponse<IngestEventResponse>>, AppError> {
    let project = state.project_service.find_project_by_id(project_id).await?;
    let jira_tool_id = project.jira_tool_id
        .ok_or_else(|| AppError(DomainError::ValidationError("Project has no Jira config linked".into())))?;
    let tool = state.tool_service.find_tool_by_id(jira_tool_id).await?;

    let config: serde_json::Value = tool.config.as_deref()
        .and_then(|c| serde_json::from_str(c).ok())
        .unwrap_or_default();
    let expected_secret = config.get("webhook_secret").and_then(|v| v.as_str())
        .ok_or_else(|| AppError(DomainError::Forbidden("Jira webhook is not configured for this project".into())))?;
    if expected_secret != secret {
        return Err(AppError(DomainError::Forbidden("Invalid webhook secret".into())));
    }

    let event_type = if payload.webhook_event.contains("issue_created") {
        "jira.ticket.created"
    } else if payload.webhook_event.contains("issue_updated") {
        "jira.ticket.updated"
    } else {
        "jira.ticket.event"
    };

    let event_payload = serde_json::json!({
        "project_id": project_id,
        "ticket_key": payload.issue.key,
        "summary": payload.issue.fields.summary,
        "description": payload.issue.fields.description,
        "issue_type": payload.issue.fields.issuetype.map(|t| t.name),
        "priority": payload.issue.fields.priority.map(|p| p.name),
    });

    let result = state.event_service.ingest_event(IngestEventCommand {
        tenant_id: project.tenant_id,
        event_type: event_type.to_string(),
        payload: event_payload,
    }).await?;

    Ok(Json(ApiResponse::created(result)))
}
