use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Mirrors sso-domain's Claims shape exactly: tokens are issued by the sso
// service and validated here against the same JWT_SECRET, so field names
// and types must stay in lockstep with sso.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,       // user id
    pub tenant_id: String, // tenant id
    pub username: String,
    pub exp: usize,
    pub iat: usize,
    pub jti: String,       // jwt id
}

impl Claims {
    pub fn user_id(&self) -> Option<Uuid> {
        self.sub.parse().ok()
    }

    pub fn tenant_uuid(&self) -> Option<Uuid> {
        self.tenant_id.parse().ok()
    }
}
