use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::AutomationRuleResponse::AutomationRuleResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::AutomationRulePayloads::{CreateAutomationRuleRequest, UpdateAutomationRuleRequest};

#[utoipa::path(
    post,
    path = "/api/v1/automation-rules",
    tag = "Automation Rules",
    request_body = CreateAutomationRuleRequest,
    responses(
        (status = 201, description = "Automation rule created", body = ApiResponse<AutomationRuleResponse>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn create_rule(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAutomationRuleRequest>,
) -> Result<Json<ApiResponse<AutomationRuleResponse>>, AppError> {
    let rule = state.rule_service.create_rule(payload.into()).await?;
    Ok(Json(ApiResponse::created(rule)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/automation-rules",
    tag = "Automation Rules",
    params(("tenant_id" = Uuid, Path, description = "Tenant ID")),
    responses(
        (status = 200, description = "List automation rules for a tenant", body = ApiResponse<Vec<AutomationRuleResponse>>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_rules_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<AutomationRuleResponse>>>, AppError> {
    let rules = state.rule_service.find_rules_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(rules)))
}

#[utoipa::path(
    get,
    path = "/api/v1/automation-rules/{id}",
    tag = "Automation Rules",
    params(("id" = Uuid, Path, description = "Automation rule ID")),
    responses(
        (status = 200, description = "Get automation rule by ID", body = ApiResponse<AutomationRuleResponse>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_rule(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AutomationRuleResponse>>, AppError> {
    let rule = state.rule_service.find_rule_by_id(id).await?;
    Ok(Json(ApiResponse::success(rule)))
}

#[utoipa::path(
    put,
    path = "/api/v1/automation-rules/{id}",
    tag = "Automation Rules",
    params(("id" = Uuid, Path, description = "Automation rule ID")),
    request_body = UpdateAutomationRuleRequest,
    responses(
        (status = 200, description = "Automation rule updated", body = ApiResponse<AutomationRuleResponse>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn update_rule(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateAutomationRuleRequest>,
) -> Result<Json<ApiResponse<AutomationRuleResponse>>, AppError> {
    let rule = state.rule_service.update_rule(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(rule)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/automation-rules/{id}",
    tag = "Automation Rules",
    params(("id" = Uuid, Path, description = "Automation rule ID")),
    responses(
        (status = 204, description = "Automation rule deleted")
    ),
    security(("bearerAuth" = []))
)]
pub async fn delete_rule(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.rule_service.delete_rule(id).await?;
    Ok(Json(ApiResponse::no_content()))
}
