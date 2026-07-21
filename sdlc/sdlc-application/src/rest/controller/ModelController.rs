use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::ModelResponse::ModelResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::ModelPayloads::{CreateModelRequest, UpdateModelRequest};

#[utoipa::path(
    post,
    path = "/api/v1/models",
    tag = "Models",
    request_body = CreateModelRequest,
    responses(
        (status = 201, description = "Model registered", body = ApiResponse<ModelResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_model(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateModelRequest>,
) -> Result<Json<ApiResponse<ModelResponse>>, AppError> {
    let model = state.model_service.create_model(payload.into()).await?;
    Ok(Json(ApiResponse::created(model)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/models",
    tag = "Models",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "List models available to a tenant (tenant-specific + global)", body = ApiResponse<Vec<ModelResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_models_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<ModelResponse>>>, AppError> {
    let models = state.model_service.find_models_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(models)))
}

#[utoipa::path(
    get,
    path = "/api/v1/models/{id}",
    tag = "Models",
    params(
        ("id" = Uuid, Path, description = "Model ID")
    ),
    responses(
        (status = 200, description = "Get model by ID", body = ApiResponse<ModelResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_model(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ModelResponse>>, AppError> {
    let model = state.model_service.find_model_by_id(id).await?;
    Ok(Json(ApiResponse::success(model)))
}

#[utoipa::path(
    put,
    path = "/api/v1/models/{id}",
    tag = "Models",
    params(
        ("id" = Uuid, Path, description = "Model ID")
    ),
    request_body = UpdateModelRequest,
    responses(
        (status = 200, description = "Model updated", body = ApiResponse<ModelResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_model(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateModelRequest>,
) -> Result<Json<ApiResponse<ModelResponse>>, AppError> {
    let model = state.model_service.update_model(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(model)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/models/{id}",
    tag = "Models",
    params(
        ("id" = Uuid, Path, description = "Model ID")
    ),
    responses(
        (status = 204, description = "Model deleted")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_model(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.model_service.delete_model(id).await?;
    Ok(Json(ApiResponse::no_content()))
}
