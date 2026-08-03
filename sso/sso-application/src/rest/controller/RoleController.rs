use std::collections::HashSet;
use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sso_domain::dto::PermissionResponse::PermissionResponse;
use sso_domain::dto::RoleResponse::RoleResponse;
use crate::casbin::CasbinSync;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::RolePayloads::{CreateRoleRequest, UpdateRoleRequest};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AssignAllPermissionsResult {
    pub assigned: usize,
}

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
    let role = state.role_service.find_role_by_id(id).await?;

    state.role_service.delete_role(id).await?;

    let mut enforcer = state.enforcer.write().await;
    CasbinSync::revoke_all_for_role(&mut enforcer, &role.name).await?;

    Ok(Json(ApiResponse::no_content()))
}

#[utoipa::path(
    get,
    path = "/api/v1/roles/{id}/permissions",
    tag = "Roles",
    params(
        ("id" = Uuid, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "List permissions assigned to a role", body = ApiResponse<Vec<PermissionResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_role_permissions(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<PermissionResponse>>>, AppError> {
    let permission_ids = state.role_service.find_permission_ids_by_role(id).await?;

    let mut permissions = Vec::with_capacity(permission_ids.len());
    for permission_id in permission_ids {
        permissions.push(state.permission_service.find_permission_by_id(permission_id).await?);
    }

    Ok(Json(ApiResponse::success(permissions)))
}

#[utoipa::path(
    post,
    path = "/api/v1/roles/{id}/permissions/all",
    tag = "Roles",
    params(
        ("id" = Uuid, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "Assign every permission in the catalog to this role that isn't already assigned", body = ApiResponse<AssignAllPermissionsResult>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn assign_all_permissions(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AssignAllPermissionsResult>>, AppError> {
    let role = state.role_service.find_role_by_id(id).await?;
    let all_permissions = state.permission_service.find_all_permissions().await?;
    let assigned_ids: HashSet<Uuid> = state.role_service.find_permission_ids_by_role(id).await?.into_iter().collect();

    let mut assigned = 0;
    for permission in all_permissions.into_iter().filter(|p| !assigned_ids.contains(&p.id)) {
        state.role_service.assign_permission(id, permission.id).await?;

        let mut enforcer = state.enforcer.write().await;
        CasbinSync::grant_permission(&mut enforcer, &role.name, &permission.resource, &permission.action).await?;

        assigned += 1;
    }

    Ok(Json(ApiResponse::success(AssignAllPermissionsResult { assigned })))
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

    let role = state.role_service.find_role_by_id(id).await?;
    let permission = state.permission_service.find_permission_by_id(permission_id).await?;
    let mut enforcer = state.enforcer.write().await;
    CasbinSync::grant_permission(&mut enforcer, &role.name, &permission.resource, &permission.action).await?;

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
    let role = state.role_service.find_role_by_id(id).await?;
    let permission = state.permission_service.find_permission_by_id(permission_id).await?;

    state.role_service.remove_permission(id, permission_id).await?;

    let mut enforcer = state.enforcer.write().await;
    CasbinSync::revoke_permission(&mut enforcer, &role.name, &permission.resource, &permission.action).await?;

    Ok(Json(ApiResponse::no_content()))
}
