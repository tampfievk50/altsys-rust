use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::SkillResponse::SkillResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::SkillPayloads::{CreateSkillRequest, UpdateSkillRequest};

#[utoipa::path(
    post,
    path = "/api/v1/skills",
    tag = "Skills",
    request_body = CreateSkillRequest,
    responses(
        (status = 201, description = "Skill registered", body = ApiResponse<SkillResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_skill(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateSkillRequest>,
) -> Result<Json<ApiResponse<SkillResponse>>, AppError> {
    let skill = state.skill_service.create_skill(payload.into()).await?;
    Ok(Json(ApiResponse::created(skill)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/skills",
    tag = "Skills",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "List skills available to a tenant (tenant-specific + global)", body = ApiResponse<Vec<SkillResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_skills_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<SkillResponse>>>, AppError> {
    let skills = state.skill_service.find_skills_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(skills)))
}

#[utoipa::path(
    get,
    path = "/api/v1/skills/{id}",
    tag = "Skills",
    params(
        ("id" = Uuid, Path, description = "Skill ID")
    ),
    responses(
        (status = 200, description = "Get skill by ID", body = ApiResponse<SkillResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_skill(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<SkillResponse>>, AppError> {
    let skill = state.skill_service.find_skill_by_id(id).await?;
    Ok(Json(ApiResponse::success(skill)))
}

#[utoipa::path(
    put,
    path = "/api/v1/skills/{id}",
    tag = "Skills",
    params(
        ("id" = Uuid, Path, description = "Skill ID")
    ),
    request_body = UpdateSkillRequest,
    responses(
        (status = 200, description = "Skill updated", body = ApiResponse<SkillResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_skill(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateSkillRequest>,
) -> Result<Json<ApiResponse<SkillResponse>>, AppError> {
    let skill = state.skill_service.update_skill(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(skill)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/skills/{id}",
    tag = "Skills",
    params(
        ("id" = Uuid, Path, description = "Skill ID")
    ),
    responses(
        (status = 204, description = "Skill deleted")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_skill(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.skill_service.delete_skill(id).await?;
    Ok(Json(ApiResponse::no_content()))
}
