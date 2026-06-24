use std::sync::Arc;
use casbin::Enforcer;
use tokio::sync::RwLock;

use sso_domain::port::input::TenantPort::TenantPort;
use sso_domain::port::input::UserPort::UserPort;
use sso_domain::port::input::RolePort::RolePort;
use sso_domain::port::input::PermissionPort::PermissionPort;
use sso_domain::port::input::AuthPort::AuthPort;

pub struct AppState {
    pub tenant_service: Arc<dyn TenantPort>,
    pub user_service: Arc<dyn UserPort>,
    pub role_service: Arc<dyn RolePort>,
    pub permission_service: Arc<dyn PermissionPort>,
    pub auth_service: Arc<dyn AuthPort>,
    pub enforcer: Arc<RwLock<Enforcer>>,
}
