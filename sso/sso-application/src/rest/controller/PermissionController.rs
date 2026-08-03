use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use serde::Serialize;
use utoipa::ToSchema;

use sso_domain::dto::CreatePermissionCommand::CreatePermissionCommand;
use sso_domain::dto::PermissionResponse::PermissionResponse;
use crate::casbin::CasbinSync;
use crate::permission_catalog::{self, PermissionCatalogDiff};
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::PermissionPayloads::{CreatePermissionRequest, UpdatePermissionRequest};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PermissionRefreshResult {
    pub created: usize,
    pub orphaned: Vec<PermissionResponse>,
}

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
    let permission = state.permission_service.find_permission_by_id(id).await?;

    state.permission_service.delete_permission(id).await?;

    let mut enforcer = state.enforcer.write().await;
    CasbinSync::revoke_all_for_permission(&mut enforcer, &permission.resource, &permission.action).await?;

    Ok(Json(ApiResponse::no_content()))
}

#[utoipa::path(
    get,
    path = "/api/v1/permissions/refresh",
    tag = "Permissions",
    responses(
        (status = 200, description = "Preview the diff between current API routes and the Permission catalog, without writing anything", body = ApiResponse<PermissionCatalogDiff>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn preview_permission_refresh(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<PermissionCatalogDiff>>, AppError> {
    let existing = state.permission_service.find_all_permissions().await?;
    Ok(Json(ApiResponse::success(permission_catalog::diff(&existing))))
}

#[utoipa::path(
    post,
    path = "/api/v1/permissions/refresh",
    tag = "Permissions",
    responses(
        (status = 200, description = "Create a Permission row for every current API route missing one; existing rows are left untouched", body = ApiResponse<PermissionRefreshResult>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn apply_permission_refresh(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<PermissionRefreshResult>>, AppError> {
    let existing = state.permission_service.find_all_permissions().await?;
    let diff = permission_catalog::diff(&existing);

    let mut created = 0;
    for entry in &diff.missing {
        state.permission_service.create_permission(CreatePermissionCommand {
            name: entry.name.clone(),
            action: entry.action.clone(),
            resource: entry.resource.clone(),
            description: entry.description.clone(),
        }).await?;
        created += 1;
    }

    Ok(Json(ApiResponse::success(PermissionRefreshResult { created, orphaned: diff.orphaned })))
}
