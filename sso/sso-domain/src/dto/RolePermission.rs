use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RolePermission {
    pub role_id: Uuid,
    pub permission_id: Uuid,
}
