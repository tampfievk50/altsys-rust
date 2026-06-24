use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub tenant_id: Uuid,
}
