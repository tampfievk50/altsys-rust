use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::KnowledgeSearchResult::KnowledgeSearchResult;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::SemanticSearchPayloads::SemanticSearchRequest;

#[utoipa::path(
    post,
    path = "/api/v1/tenants/{tenant_id}/knowledge-items/search",
    tag = "Knowledge Search",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID")
    ),
    request_body = SemanticSearchRequest,
    responses(
        (status = 200, description = "Knowledge items ranked by semantic similarity to the query", body = ApiResponse<Vec<KnowledgeSearchResult>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn search_knowledge(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
    Json(payload): Json<SemanticSearchRequest>,
) -> Result<Json<ApiResponse<Vec<KnowledgeSearchResult>>>, AppError> {
    let results = state.search_service.search(tenant_id, payload.into()).await?;
    Ok(Json(ApiResponse::success(results)))
}
