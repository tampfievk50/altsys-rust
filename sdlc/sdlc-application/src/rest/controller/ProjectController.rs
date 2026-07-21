use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::ProjectResponse::ProjectResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::ProjectPayloads::{CreateProjectRequest, UpdateProjectRequest};

#[utoipa::path(
    post,
    path = "/api/v1/projects",
    tag = "Projects",
    request_body = CreateProjectRequest,
    responses(
        (status = 201, description = "Project created", body = ApiResponse<ProjectResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_project(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<Json<ApiResponse<ProjectResponse>>, AppError> {
    let project = state.project_service.create_project(payload.into()).await?;
    Ok(Json(ApiResponse::created(project)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/projects",
    tag = "Projects",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "List projects by tenant", body = ApiResponse<Vec<ProjectResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_projects_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<ProjectResponse>>>, AppError> {
    let projects = state.project_service.find_projects_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(projects)))
}

#[utoipa::path(
    get,
    path = "/api/v1/projects/{id}",
    tag = "Projects",
    params(
        ("id" = Uuid, Path, description = "Project ID")
    ),
    responses(
        (status = 200, description = "Get project by ID", body = ApiResponse<ProjectResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ProjectResponse>>, AppError> {
    let project = state.project_service.find_project_by_id(id).await?;
    Ok(Json(ApiResponse::success(project)))
}

#[utoipa::path(
    put,
    path = "/api/v1/projects/{id}",
    tag = "Projects",
    params(
        ("id" = Uuid, Path, description = "Project ID")
    ),
    request_body = UpdateProjectRequest,
    responses(
        (status = 200, description = "Project updated", body = ApiResponse<ProjectResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateProjectRequest>,
) -> Result<Json<ApiResponse<ProjectResponse>>, AppError> {
    let project = state.project_service.update_project(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(project)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/projects/{id}",
    tag = "Projects",
    params(
        ("id" = Uuid, Path, description = "Project ID")
    ),
    responses(
        (status = 204, description = "Project deleted")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.project_service.delete_project(id).await?;
    Ok(Json(ApiResponse::no_content()))
}
