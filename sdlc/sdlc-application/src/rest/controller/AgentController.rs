use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::AgentResponse::AgentResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::AgentPayloads::{CreateAgentRequest, UpdateAgentRequest};

#[utoipa::path(
    post,
    path = "/api/v1/agents",
    tag = "Agents",
    request_body = CreateAgentRequest,
    responses(
        (status = 201, description = "Agent registered", body = ApiResponse<AgentResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_agent(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAgentRequest>,
) -> Result<Json<ApiResponse<AgentResponse>>, AppError> {
    let agent = state.agent_service.create_agent(payload.into()).await?;
    Ok(Json(ApiResponse::created(agent)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/agents",
    tag = "Agents",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "List agents available to a tenant (tenant-specific + global)", body = ApiResponse<Vec<AgentResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_agents_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<AgentResponse>>>, AppError> {
    let agents = state.agent_service.find_agents_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(agents)))
}

#[utoipa::path(
    get,
    path = "/api/v1/agents/{id}",
    tag = "Agents",
    params(
        ("id" = Uuid, Path, description = "Agent ID")
    ),
    responses(
        (status = 200, description = "Get agent by ID", body = ApiResponse<AgentResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_agent(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AgentResponse>>, AppError> {
    let agent = state.agent_service.find_agent_by_id(id).await?;
    Ok(Json(ApiResponse::success(agent)))
}

#[utoipa::path(
    put,
    path = "/api/v1/agents/{id}",
    tag = "Agents",
    params(
        ("id" = Uuid, Path, description = "Agent ID")
    ),
    request_body = UpdateAgentRequest,
    responses(
        (status = 200, description = "Agent updated", body = ApiResponse<AgentResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_agent(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateAgentRequest>,
) -> Result<Json<ApiResponse<AgentResponse>>, AppError> {
    let agent = state.agent_service.update_agent(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(agent)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/agents/{id}",
    tag = "Agents",
    params(
        ("id" = Uuid, Path, description = "Agent ID")
    ),
    responses(
        (status = 204, description = "Agent deleted")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_agent(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.agent_service.delete_agent(id).await?;
    Ok(Json(ApiResponse::no_content()))
}
