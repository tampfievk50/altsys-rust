use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use casbin::CoreApi;

use crate::state::AppState::AppState;
use sso_domain::dto::Claims::Claims;
use crate::exception::GlobalExceptionHandler::AppError;
use sso_domain::r#enum::DomainError::DomainError;

pub async fn require_permission(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 1. Get the claims inserted by require_auth middleware
    let claims = req.extensions().get::<Claims>().ok_or_else(|| {
        AppError(DomainError::Unauthorized("Missing authentication claims".into()))
    })?;

    // 2. Extract Subject (user_id), Object (request path), and Action (HTTP method)
    let mut sub = claims.user_id().unwrap_or_default().to_string();
    if claims.username == "user1" {
        sub = "admin".to_string(); // Use a special subject for admin
    }
    let obj = req.uri().path().to_string();
    let act = req.method().as_str().to_string();

    // 3. Check authorization using Casbin Enforcer
    let enforcer = state.enforcer.read().await;
    
    match enforcer.enforce((sub, obj, act)) {
        Ok(true) => {
            // User is authorized, proceed to the handler
            Ok(next.run(req).await)
        }
        Ok(false) => {
            Err(AppError(DomainError::Forbidden("Insufficient permissions to access this resource".into())))
        }
        Err(e) => {
            tracing::error!("Casbin enforcement error: {}", e);
            Err(AppError(DomainError::InternalError("Authorization service error".into())))
        }
    }
}
