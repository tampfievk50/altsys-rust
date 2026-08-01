use axum::{
    extract::Request,
    http::header,
    middleware::Next,
    response::Response,
};

use scheduler_domain::util::TokenValidator::TokenValidator;
use crate::exception::GlobalExceptionHandler::AppError;
use scheduler_domain::r#enum::DomainError::DomainError;

// Scheduler validates tokens issued by sso; it does not issue or refresh
// tokens itself, so there is no local user lookup here (unlike sso's
// AuthMiddleware, which also checks the user is still active in its DB).
pub async fn require_auth(mut req: Request, next: Next) -> Result<Response, AppError> {
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

    let claims = TokenValidator::validate_token(token)?;
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
