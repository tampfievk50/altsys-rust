use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::EventResponse::EventResponse;
use sdlc_domain::dto::IngestEventResponse::IngestEventResponse;
use sdlc_domain::dto::RuleFiringResponse::RuleFiringResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::EventPayloads::IngestEventRequest;

#[utoipa::path(
    post,
    path = "/api/v1/tenants/{tenant_id}/events",
    tag = "Events",
    params(("tenant_id" = Uuid, Path, description = "Tenant ID")),
    request_body = IngestEventRequest,
    responses(
        (status = 201, description = "Event ingested and evaluated against every active automation rule", body = ApiResponse<IngestEventResponse>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn ingest_event(
    State(state): State<Arc<AppState>>,
    Path(_tenant_id): Path<Uuid>,
    Json(payload): Json<IngestEventRequest>,
) -> Result<Json<ApiResponse<IngestEventResponse>>, AppError> {
    let result = state.event_service.ingest_event(payload.into()).await?;
    Ok(Json(ApiResponse::created(result)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/events",
    tag = "Events",
    params(("tenant_id" = Uuid, Path, description = "Tenant ID")),
    responses(
        (status = 200, description = "List ingested events for a tenant", body = ApiResponse<Vec<EventResponse>>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_events_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<EventResponse>>>, AppError> {
    let events = state.event_service.find_events_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(events)))
}

#[utoipa::path(
    get,
    path = "/api/v1/events/{id}/firings",
    tag = "Events",
    params(("id" = Uuid, Path, description = "Event ID")),
    responses(
        (status = 200, description = "Every rule evaluated against this event and its outcome", body = ApiResponse<Vec<RuleFiringResponse>>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_firings_by_event(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<RuleFiringResponse>>>, AppError> {
    let firings = state.event_service.find_firings_by_event(id).await?;
    Ok(Json(ApiResponse::success(firings)))
}
