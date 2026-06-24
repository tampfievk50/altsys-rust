use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use tracing::info;
use uuid::Uuid;

use crawler_domain::dto::CreateUserCommand::CreateUserCommand;
use crawler_domain::dto::UpdateUserCommand::UpdateUserCommand;
#[allow(unused_imports)]
use crawler_domain::dto::UserResponse::UserResponse;

use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::payload::CreateUserPayload::CreateUserPayload;
use crate::rest::payload::UpdateUserPayload::UpdateUserPayload;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::state::AppState::AppState;

#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "Users",
    request_body = CreateUserPayload,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Invalid request body"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<impl IntoResponse, AppError> {
    info!(username = %payload.username, "Received create user request");

    let command = CreateUserCommand {
        username: payload.username,
        email: payload.email,
        password: payload.password,
        full_name: payload.full_name,
    };

    let response = state.user_service.create_user(command).await.map_err(AppError::from)?;

    Ok((StatusCode::CREATED, Json(ApiResponse::success(response))))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "User UUID")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    info!(user_id = %id, "Fetching user by ID");

    let response = state.user_service.find_user_by_id(id).await.map_err(AppError::from)?;

    Ok(Json(ApiResponse::success(response)))
}

#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "Users",
    responses(
        (status = 200, description = "List of users", body = Vec<UserResponse>),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn list_users(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    info!("Listing all users");

    let response = state.user_service.find_all_users().await.map_err(AppError::from)?;

    Ok(Json(ApiResponse::success(response)))
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "User UUID")
    ),
    request_body = UpdateUserPayload,
    responses(
        (status = 200, description = "User updated successfully", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<impl IntoResponse, AppError> {
    info!(user_id = %id, "Received update user request");

    let command = UpdateUserCommand {
        email: payload.email,
        password: payload.password,
        full_name: payload.full_name,
        is_active: payload.is_active,
    };

    let response = state.user_service.update_user(id, command).await.map_err(AppError::from)?;

    Ok(Json(ApiResponse::success(response)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    tag = "Users",
    params(
        ("id" = Uuid, Path, description = "User UUID")
    ),
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    info!(user_id = %id, "Received delete user request");

    state.user_service.delete_user(id).await.map_err(AppError::from)?;

    Ok((StatusCode::NO_CONTENT, Json(ApiResponse::success(()))))
}
