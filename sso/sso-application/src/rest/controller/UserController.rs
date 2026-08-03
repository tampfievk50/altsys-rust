use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sso_domain::dto::RoleResponse::RoleResponse;
use sso_domain::dto::UserResponse::UserResponse;
use crate::casbin::CasbinSync;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::UserPayloads::{CreateUserRequest, UpdateUserRequest};

#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "Users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = ApiResponse<UserResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = state.user_service.create_user(payload.into()).await?;
    Ok(Json(ApiResponse::created(user)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/users",
    tag = "Users",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "List users by tenant", body = ApiResponse<Vec<UserResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_users_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<UserResponse>>>, AppError> {
    let users = state.user_service.find_users_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(users)))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Get user by ID", body = ApiResponse<UserResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = state.user_service.find_user_by_id(id).await?;
    Ok(Json(ApiResponse::success(user)))
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = ApiResponse<UserResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = state.user_service.update_user(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(user)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deleted")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.user_service.delete_user(id).await?;

    let mut enforcer = state.enforcer.write().await;
    CasbinSync::revoke_all_for_user(&mut enforcer, &id.to_string()).await?;

    Ok(Json(ApiResponse::no_content()))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}/roles",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "List roles assigned to a user", body = ApiResponse<Vec<RoleResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_user_roles(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<RoleResponse>>>, AppError> {
    let role_ids = state.user_service.find_role_ids_by_user(id).await?;

    let mut roles = Vec::with_capacity(role_ids.len());
    for role_id in role_ids {
        roles.push(state.role_service.find_role_by_id(role_id).await?);
    }

    Ok(Json(ApiResponse::success(roles)))
}

#[utoipa::path(
    post,
    path = "/api/v1/users/{id}/roles/{role_id}/{tenant_id}",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "User ID"),
        ("role_id" = Uuid, Path, description = "Role ID"),
        ("tenant_id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "Role assigned to user")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn assign_role(
    State(state): State<Arc<AppState>>,
    Path((id, role_id, tenant_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.user_service.assign_role(id, role_id, tenant_id).await?;

    let role = state.role_service.find_role_by_id(role_id).await?;
    let mut enforcer = state.enforcer.write().await;
    CasbinSync::grant_role(&mut enforcer, &id.to_string(), &role.name).await?;

    Ok(Json(ApiResponse::success(())))
}

#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}/roles/{role_id}",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "User ID"),
        ("role_id" = Uuid, Path, description = "Role ID")
    ),
    responses(
        (status = 204, description = "Role removed from user")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn remove_role(
    State(state): State<Arc<AppState>>,
    Path((id, role_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let role = state.role_service.find_role_by_id(role_id).await?;

    state.user_service.remove_role(id, role_id).await?;

    let mut enforcer = state.enforcer.write().await;
    CasbinSync::revoke_role(&mut enforcer, &id.to_string(), &role.name).await?;

    Ok(Json(ApiResponse::no_content()))
}
