use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sso_domain::dto::PermissionResponse::PermissionResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::PermissionPayloads::{CreatePermissionRequest, UpdatePermissionRequest};

#[utoipa::path(
    post,
    path = "/api/v1/permissions",
    tag = "Permissions",
    request_body = CreatePermissionRequest,
    responses(
        (status = 201, description = "Permission created", body = ApiResponse<PermissionResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_permission(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePermissionRequest>,
) -> Result<Json<ApiResponse<PermissionResponse>>, AppError> {
    let permission = state.permission_service.create_permission(payload.into()).await?;
    Ok(Json(ApiResponse::created(permission)))
}

#[utoipa::path(
    get,
    path = "/api/v1/permissions",
    tag = "Permissions",
    responses(
        (status = 200, description = "List all permissions", body = ApiResponse<Vec<PermissionResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_all_permissions(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<PermissionResponse>>>, AppError> {
    let permissions = state.permission_service.find_all_permissions().await?;
    Ok(Json(ApiResponse::success(permissions)))
}

#[utoipa::path(
    get,
    path = "/api/v1/permissions/{id}",
    tag = "Permissions",
    params(
        ("id" = Uuid, Path, description = "Permission ID")
    ),
    responses(
        (status = 200, description = "Get permission by ID", body = ApiResponse<PermissionResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_permission(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<PermissionResponse>>, AppError> {
    let permission = state.permission_service.find_permission_by_id(id).await?;
    Ok(Json(ApiResponse::success(permission)))
}

#[utoipa::path(
    put,
    path = "/api/v1/permissions/{id}",
    tag = "Permissions",
    params(
        ("id" = Uuid, Path, description = "Permission ID")
    ),
    request_body = UpdatePermissionRequest,
    responses(
        (status = 200, description = "Permission updated", body = ApiResponse<PermissionResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_permission(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePermissionRequest>,
) -> Result<Json<ApiResponse<PermissionResponse>>, AppError> {
    let permission = state.permission_service.update_permission(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(permission)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/permissions/{id}",
    tag = "Permissions",
    params(
        ("id" = Uuid, Path, description = "Permission ID")
    ),
    responses(
        (status = 204, description = "Permission deleted")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_permission(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.permission_service.delete_permission(id).await?;
    Ok(Json(ApiResponse::no_content()))
}
