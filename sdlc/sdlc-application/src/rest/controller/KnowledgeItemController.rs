use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::KnowledgeItemResponse::KnowledgeItemResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::KnowledgeItemPayloads::{CreateKnowledgeItemRequest, UpdateKnowledgeItemRequest};

#[utoipa::path(
    post,
    path = "/api/v1/knowledge-items",
    tag = "Knowledge",
    request_body = CreateKnowledgeItemRequest,
    responses(
        (status = 201, description = "Knowledge item version created", body = ApiResponse<KnowledgeItemResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_knowledge_item(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateKnowledgeItemRequest>,
) -> Result<Json<ApiResponse<KnowledgeItemResponse>>, AppError> {
    let item = state.knowledge_service.create_knowledge_item(payload.into()).await?;
    Ok(Json(ApiResponse::created(item)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/knowledge-items",
    tag = "Knowledge",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "List the latest version of every knowledge item key for a tenant", body = ApiResponse<Vec<KnowledgeItemResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_knowledge_items_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<KnowledgeItemResponse>>>, AppError> {
    let items = state.knowledge_service.find_knowledge_items_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(items)))
}

#[utoipa::path(
    get,
    path = "/api/v1/knowledge-items/{id}",
    tag = "Knowledge",
    params(
        ("id" = Uuid, Path, description = "Knowledge item ID")
    ),
    responses(
        (status = 200, description = "Get a knowledge item version by ID", body = ApiResponse<KnowledgeItemResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_knowledge_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<KnowledgeItemResponse>>, AppError> {
    let item = state.knowledge_service.find_knowledge_item_by_id(id).await?;
    Ok(Json(ApiResponse::success(item)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/knowledge-items/{key}/latest",
    tag = "Knowledge",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID"),
        ("key" = String, Path, description = "Knowledge item key")
    ),
    responses(
        (status = 200, description = "Get the latest version for a key", body = ApiResponse<KnowledgeItemResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_latest_knowledge_item_by_key(
    State(state): State<Arc<AppState>>,
    Path((tenant_id, key)): Path<(Uuid, String)>,
) -> Result<Json<ApiResponse<KnowledgeItemResponse>>, AppError> {
    let item = state.knowledge_service.find_latest_knowledge_item_by_key(tenant_id, &key).await?;
    Ok(Json(ApiResponse::success(item)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/knowledge-items/{key}/versions",
    tag = "Knowledge",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID"),
        ("key" = String, Path, description = "Knowledge item key")
    ),
    responses(
        (status = 200, description = "List all versions for a key, oldest first", body = ApiResponse<Vec<KnowledgeItemResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_knowledge_item_versions_by_key(
    State(state): State<Arc<AppState>>,
    Path((tenant_id, key)): Path<(Uuid, String)>,
) -> Result<Json<ApiResponse<Vec<KnowledgeItemResponse>>>, AppError> {
    let versions = state.knowledge_service.find_knowledge_item_versions_by_key(tenant_id, &key).await?;
    Ok(Json(ApiResponse::success(versions)))
}

#[utoipa::path(
    put,
    path = "/api/v1/knowledge-items/{id}",
    tag = "Knowledge",
    params(
        ("id" = Uuid, Path, description = "Knowledge item ID")
    ),
    request_body = UpdateKnowledgeItemRequest,
    responses(
        (status = 200, description = "Knowledge item updated in place (re-embedded if content changed)", body = ApiResponse<KnowledgeItemResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_knowledge_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateKnowledgeItemRequest>,
) -> Result<Json<ApiResponse<KnowledgeItemResponse>>, AppError> {
    let item = state.knowledge_service.update_knowledge_item(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(item)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/knowledge-items/{id}",
    tag = "Knowledge",
    params(
        ("id" = Uuid, Path, description = "Knowledge item ID")
    ),
    responses(
        (status = 204, description = "Knowledge item version deleted")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_knowledge_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.knowledge_service.delete_knowledge_item(id).await?;
    Ok(Json(ApiResponse::no_content()))
}
