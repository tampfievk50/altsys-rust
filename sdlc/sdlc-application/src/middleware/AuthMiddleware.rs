use std::env;
use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use sdlc_domain::r#enum::DomainError::DomainError;

use crate::exception::GlobalExceptionHandler::AppError;
use crate::state::AppState::AppState;

/// JWT claims issued by the SSO service. Verified here (stateless, shared
/// JWT_SECRET) since this service has no local users table to check against.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub tenant_id: String,
    pub username: String,
    pub exp: usize,
    pub iat: usize,
    pub jti: String,
}

fn jwt_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| "default-secret-change-me".into())
}

pub async fn require_auth(
    State(_state): State<Arc<AppState>>,
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

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError(DomainError::Unauthorized(format!("Invalid token: {}", e))))?;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
