use std::sync::Arc;
use axum::{extract::State, Json};

use sso_domain::dto::TokenResponse::TokenResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::AuthPayloads::{LoginRequest, RefreshRequest, LogoutRequest};

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<TokenResponse>)
    )
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<TokenResponse>>, AppError> {
    let token_response = state.auth_service.login(payload.into()).await?;
    Ok(Json(ApiResponse::success(token_response)))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "Auth",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Tokens refreshed", body = ApiResponse<TokenResponse>)
    )
)]
pub async fn refresh(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<ApiResponse<TokenResponse>>, AppError> {
    let token_response = state.auth_service.refresh(&payload.refresh_token).await?;
    Ok(Json(ApiResponse::success(token_response)))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "Auth",
    request_body = LogoutRequest,
    responses(
        (status = 204, description = "Logout successful")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn logout(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LogoutRequest>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.auth_service.logout(&payload.refresh_token).await?;
    Ok(Json(ApiResponse::no_content()))
}
