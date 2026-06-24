use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use sso_domain::r#enum::DomainError::DomainError;
use crate::rest::response::ApiResponse::ApiResponse;

pub struct AppError(pub DomainError);

impl From<DomainError> for AppError {
    fn from(err: DomainError) -> Self {
        AppError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, body) = match self.0 {
            DomainError::NotFound(msg) => (StatusCode::NOT_FOUND, ApiResponse::<()>::error(404, msg)),
            DomainError::ValidationError(msg) => (StatusCode::BAD_REQUEST, ApiResponse::<()>::error(400, msg)),
            DomainError::AlreadyExists(msg) => (StatusCode::CONFLICT, ApiResponse::<()>::error(409, msg)),
            DomainError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, ApiResponse::<()>::error(401, msg)),
            DomainError::Forbidden(msg) => (StatusCode::FORBIDDEN, ApiResponse::<()>::error(403, msg)),
            DomainError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, ApiResponse::<()>::error(500, msg)),
        };
        (status, Json(body)).into_response()
    }
}
