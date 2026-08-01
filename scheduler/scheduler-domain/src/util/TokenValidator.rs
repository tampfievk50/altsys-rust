use std::env;

use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::dto::Claims::Claims;
use crate::r#enum::DomainError::DomainError;

// Scheduler is a resource server only: it never issues tokens, it just
// validates tokens issued by sso against the same shared JWT_SECRET.
pub struct TokenValidator;

impl TokenValidator {
    fn jwt_secret() -> String {
        env::var("JWT_SECRET").unwrap_or_else(|_| "default-secret-change-me".into())
    }

    pub fn validate_token(token: &str) -> Result<Claims, DomainError> {
        let secret = Self::jwt_secret();
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| DomainError::Unauthorized(format!("Invalid token: {}", e)))
    }
}
