use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sso_domain::dto::TenantResponse::TenantResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::TenantPayloads::{CreateTenantRequest, UpdateTenantRequest};

#[utoipa::path(
    post,
    path = "/api/v1/tenants",
    tag = "Tenants",
    request_body = CreateTenantRequest,
    responses(
        (status = 201, description = "Tenant created", body = ApiResponse<TenantResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_tenant(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTenantRequest>,
) -> Result<Json<ApiResponse<TenantResponse>>, AppError> {
    let tenant = state.tenant_service.create_tenant(payload.into()).await?;
    Ok(Json(ApiResponse::created(tenant)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants",
    tag = "Tenants",
    responses(
        (status = 200, description = "List all tenants", body = ApiResponse<Vec<TenantResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_all_tenants(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<TenantResponse>>>, AppError> {
    let tenants = state.tenant_service.find_all_tenants().await?;
    Ok(Json(ApiResponse::success(tenants)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{id}",
    tag = "Tenants",
    params(
        ("id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "Get tenant by ID", body = ApiResponse<TenantResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_tenant(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<TenantResponse>>, AppError> {
    let tenant = state.tenant_service.find_tenant_by_id(id).await?;
    Ok(Json(ApiResponse::success(tenant)))
}

#[utoipa::path(
    put,
    path = "/api/v1/tenants/{id}",
    tag = "Tenants",
    params(
        ("id" = Uuid, Path, description = "Tenant ID")
    ),
    request_body = UpdateTenantRequest,
    responses(
        (status = 200, description = "Tenant updated", body = ApiResponse<TenantResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_tenant(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTenantRequest>,
) -> Result<Json<ApiResponse<TenantResponse>>, AppError> {
    let tenant = state.tenant_service.update_tenant(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(tenant)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/tenants/{id}",
    tag = "Tenants",
    params(
        ("id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 204, description = "Tenant deleted")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_tenant(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.tenant_service.delete_tenant(id).await?;
    Ok(Json(ApiResponse::no_content()))
}
