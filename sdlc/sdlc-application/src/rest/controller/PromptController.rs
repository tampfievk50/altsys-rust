use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::PromptResponse::PromptResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::PromptPayloads::{CreatePromptRequest, UpdatePromptRequest};

#[utoipa::path(
    post,
    path = "/api/v1/prompts",
    tag = "Prompts",
    request_body = CreatePromptRequest,
    responses(
        (status = 201, description = "Prompt version created", body = ApiResponse<PromptResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_prompt(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePromptRequest>,
) -> Result<Json<ApiResponse<PromptResponse>>, AppError> {
    let prompt = state.prompt_service.create_prompt(payload.into()).await?;
    Ok(Json(ApiResponse::created(prompt)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/prompts",
    tag = "Prompts",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "List latest version of every prompt key for a tenant", body = ApiResponse<Vec<PromptResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_prompts_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<PromptResponse>>>, AppError> {
    let prompts = state.prompt_service.find_prompts_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(prompts)))
}

#[utoipa::path(
    get,
    path = "/api/v1/prompts/{id}",
    tag = "Prompts",
    params(
        ("id" = Uuid, Path, description = "Prompt ID")
    ),
    responses(
        (status = 200, description = "Get prompt by ID", body = ApiResponse<PromptResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_prompt(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<PromptResponse>>, AppError> {
    let prompt = state.prompt_service.find_prompt_by_id(id).await?;
    Ok(Json(ApiResponse::success(prompt)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/prompts/{key}/latest",
    tag = "Prompts",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID"),
        ("key" = String, Path, description = "Prompt key")
    ),
    responses(
        (status = 200, description = "Get the latest version of a prompt key", body = ApiResponse<PromptResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_latest_prompt_by_key(
    State(state): State<Arc<AppState>>,
    Path((tenant_id, key)): Path<(Uuid, String)>,
) -> Result<Json<ApiResponse<PromptResponse>>, AppError> {
    let prompt = state.prompt_service.find_latest_prompt_by_key(tenant_id, &key).await?;
    Ok(Json(ApiResponse::success(prompt)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/prompts/{key}/versions",
    tag = "Prompts",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID"),
        ("key" = String, Path, description = "Prompt key")
    ),
    responses(
        (status = 200, description = "List all versions of a prompt key, oldest first", body = ApiResponse<Vec<PromptResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_prompt_versions_by_key(
    State(state): State<Arc<AppState>>,
    Path((tenant_id, key)): Path<(Uuid, String)>,
) -> Result<Json<ApiResponse<Vec<PromptResponse>>>, AppError> {
    let versions = state.prompt_service.find_prompt_versions_by_key(tenant_id, &key).await?;
    Ok(Json(ApiResponse::success(versions)))
}

#[utoipa::path(
    put,
    path = "/api/v1/prompts/{id}",
    tag = "Prompts",
    params(
        ("id" = Uuid, Path, description = "Prompt ID")
    ),
    request_body = UpdatePromptRequest,
    responses(
        (status = 200, description = "Prompt updated", body = ApiResponse<PromptResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_prompt(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePromptRequest>,
) -> Result<Json<ApiResponse<PromptResponse>>, AppError> {
    let prompt = state.prompt_service.update_prompt(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(prompt)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/prompts/{id}",
    tag = "Prompts",
    params(
        ("id" = Uuid, Path, description = "Prompt ID")
    ),
    responses(
        (status = 204, description = "Prompt deleted")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_prompt(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.prompt_service.delete_prompt(id).await?;
    Ok(Json(ApiResponse::no_content()))
}
