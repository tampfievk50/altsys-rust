use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::CredentialResponse::CredentialResponse;
use sdlc_domain::dto::CredentialSecretResponse::CredentialSecretResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::CredentialPayloads::{CreateCredentialRequest, UpdateCredentialRequest};

#[utoipa::path(
    post,
    path = "/api/v1/credentials",
    tag = "Credentials",
    request_body = CreateCredentialRequest,
    responses(
        (status = 201, description = "Credential created", body = ApiResponse<CredentialResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_credential(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCredentialRequest>,
) -> Result<Json<ApiResponse<CredentialResponse>>, AppError> {
    let credential = state.credential_service.create_credential(payload.into()).await?;
    Ok(Json(ApiResponse::created(credential)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/credentials",
    tag = "Credentials",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "List credentials by tenant", body = ApiResponse<Vec<CredentialResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_credentials_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<CredentialResponse>>>, AppError> {
    let credentials = state.credential_service.find_credentials_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(credentials)))
}

#[utoipa::path(
    get,
    path = "/api/v1/credentials/{id}",
    tag = "Credentials",
    params(
        ("id" = Uuid, Path, description = "Credential ID")
    ),
    responses(
        (status = 200, description = "Get credential by ID", body = ApiResponse<CredentialResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_credential(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<CredentialResponse>>, AppError> {
    let credential = state.credential_service.find_credential_by_id(id).await?;
    Ok(Json(ApiResponse::success(credential)))
}

#[utoipa::path(
    get,
    path = "/api/v1/credentials/{id}/secret",
    tag = "Credentials",
    params(
        ("id" = Uuid, Path, description = "Credential ID")
    ),
    responses(
        (status = 200, description = "Reveal the decrypted credential secret", body = ApiResponse<CredentialSecretResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn reveal_credential_secret(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<CredentialSecretResponse>>, AppError> {
    let secret = state.credential_service.reveal_credential_secret(id).await?;
    Ok(Json(ApiResponse::success(secret)))
}

#[utoipa::path(
    put,
    path = "/api/v1/credentials/{id}",
    tag = "Credentials",
    params(
        ("id" = Uuid, Path, description = "Credential ID")
    ),
    request_body = UpdateCredentialRequest,
    responses(
        (status = 200, description = "Credential updated", body = ApiResponse<CredentialResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_credential(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCredentialRequest>,
) -> Result<Json<ApiResponse<CredentialResponse>>, AppError> {
    let credential = state.credential_service.update_credential(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(credential)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/credentials/{id}",
    tag = "Credentials",
    params(
        ("id" = Uuid, Path, description = "Credential ID")
    ),
    responses(
        (status = 204, description = "Credential deleted")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_credential(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.credential_service.delete_credential(id).await?;
    Ok(Json(ApiResponse::no_content()))
}
