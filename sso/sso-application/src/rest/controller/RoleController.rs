use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sso_domain::dto::RoleResponse::RoleResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::RolePayloads::{CreateRoleRequest, UpdateRoleRequest};

#[utoipa::path(
    post,
    path = "/api/v1/roles",
    tag = "Roles",
    request_body = CreateRoleRequest,
    responses(
        (status = 201, description = "Role created", body = ApiResponse<RoleResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_role(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<Json<ApiResponse<RoleResponse>>, AppError> {
    let role = state.role_service.create_role(payload.into()).await?;
    Ok(Json(ApiResponse::created(role)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/roles",
    tag = "Roles",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "List roles by tenant", body = ApiResponse<Vec<RoleResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_roles_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<RoleResponse>>>, AppError> {
    let roles = state.role_service.find_roles_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(roles)))
}

#[utoipa::path(
    get,
    path = "/api/v1/roles/{id}",
    tag = "Roles",
    params(
        ("id" = Uuid, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "Get role by ID", body = ApiResponse<RoleResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<RoleResponse>>, AppError> {
    let role = state.role_service.find_role_by_id(id).await?;
    Ok(Json(ApiResponse::success(role)))
}

#[utoipa::path(
    put,
    path = "/api/v1/roles/{id}",
    tag = "Roles",
    params(
        ("id" = Uuid, Path, description = "Role ID")
    ),
    request_body = UpdateRoleRequest,
    responses(
        (status = 200, description = "Role updated", body = ApiResponse<RoleResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<ApiResponse<RoleResponse>>, AppError> {
    let role = state.role_service.update_role(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(role)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/roles/{id}",
    tag = "Roles",
    params(
        ("id" = Uuid, Path, description = "Role ID")
    ),
    responses(
        (status = 204, description = "Role deleted")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.role_service.delete_role(id).await?;
    Ok(Json(ApiResponse::no_content()))
}

#[utoipa::path(
    post,
    path = "/api/v1/roles/{id}/permissions/{permission_id}",
    tag = "Roles",
    params(
        ("id" = Uuid, Path, description = "Role ID"),
        ("permission_id" = Uuid, Path, description = "Permission ID")
    ),
    responses(
        (status = 200, description = "Permission assigned to role")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn assign_permission(
    State(state): State<Arc<AppState>>,
    Path((id, permission_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.role_service.assign_permission(id, permission_id).await?;
    Ok(Json(ApiResponse::success(())))
}

#[utoipa::path(
    delete,
    path = "/api/v1/roles/{id}/permissions/{permission_id}",
    tag = "Roles",
    params(
        ("id" = Uuid, Path, description = "Role ID"),
        ("permission_id" = Uuid, Path, description = "Permission ID")
    ),
    responses(
        (status = 204, description = "Permission removed from role")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn remove_permission(
    State(state): State<Arc<AppState>>,
    Path((id, permission_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.role_service.remove_permission(id, permission_id).await?;
    Ok(Json(ApiResponse::no_content()))
}
