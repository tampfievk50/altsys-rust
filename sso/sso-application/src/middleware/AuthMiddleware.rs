use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;
use tracing::error;

use sso_domain::service::AuthService::AuthService;
use crate::state::AppState::AppState;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::exception::GlobalExceptionHandler::AppError;
use sso_domain::r#enum::DomainError::DomainError;

pub async fn require_auth(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req.headers().get(header::AUTHORIZATION);
    
    let token = match auth_header {
        Some(value) => {
            let str_val = value.to_str().unwrap_or("");
            if !str_val.starts_with("Bearer ") {
                return Err(AppError(DomainError::Unauthorized("Invalid authorization header format".into())));
            }
            &str_val[7..]
        }
        None => return Err(AppError(DomainError::Unauthorized("Missing authorization header".into()))),
    };

    let claims = AuthService::validate_token(token)?;
    
    // Check if tenant and user exist and are active (optional but good for security)
    let user_id = claims.user_id().ok_or_else(|| AppError(DomainError::Unauthorized("Invalid user id in token".into())))?;
    
    let user = state.user_service.find_user_by_id(user_id).await
        .map_err(|_| AppError(DomainError::Unauthorized("User not found".into())))?;
        
    if !user.is_active {
        return Err(AppError(DomainError::Forbidden("User is inactive".into())));
    }

    // Insert claims into request extensions so handlers can access them
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
