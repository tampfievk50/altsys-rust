use std::sync::Arc;
use casbin::Enforcer;
use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;

use crate::state::AppState::AppState;

use sso_dataaccess::tenant::adapter::TenantRepositoryImpl::TenantRepositoryImpl;
use sso_dataaccess::tenant::repository::TenantSeaOrmRepository::TenantSeaOrmRepository;
use sso_dataaccess::user::adapter::UserRepositoryImpl::UserRepositoryImpl;
use sso_dataaccess::user::repository::UserSeaOrmRepository::UserSeaOrmRepository;
use sso_dataaccess::role::adapter::RoleRepositoryImpl::RoleRepositoryImpl;
use sso_dataaccess::role::repository::RoleSeaOrmRepository::RoleSeaOrmRepository;
use sso_dataaccess::permission::adapter::PermissionRepositoryImpl::PermissionRepositoryImpl;
use sso_dataaccess::permission::repository::PermissionSeaOrmRepository::PermissionSeaOrmRepository;
use sso_dataaccess::refresh_token::adapter::RefreshTokenRepositoryImpl::RefreshTokenRepositoryImpl;
use sso_dataaccess::refresh_token::repository::RefreshTokenSeaOrmRepository::RefreshTokenSeaOrmRepository;

use sso_domain::port::input::TenantPort::TenantPort;
use sso_domain::port::input::UserPort::UserPort;
use sso_domain::port::input::RolePort::RolePort;
use sso_domain::port::input::PermissionPort::PermissionPort;
use sso_domain::port::input::AuthPort::AuthPort;

use sso_domain::service::TenantService::TenantService;
use sso_domain::service::UserService::UserService;
use sso_domain::service::RoleService::RoleService;
use sso_domain::service::PermissionService::PermissionService;
use sso_domain::service::AuthService::AuthService;

pub async fn create_app_state(db: DatabaseConnection, enforcer: Arc<RwLock<Enforcer>>) -> Arc<AppState> {
    // Tenant wiring
    let tenant_repo = Arc::new(TenantRepositoryImpl::new(TenantSeaOrmRepository::new(db.clone())));
    let tenant_service = Arc::new(TenantService::new(tenant_repo.clone())) as Arc<dyn TenantPort>;

    // User wiring
    let user_repo = Arc::new(UserRepositoryImpl::new(UserSeaOrmRepository::new(db.clone())));
    let user_service = Arc::new(UserService::new(user_repo.clone())) as Arc<dyn UserPort>;

    // Role wiring
    let role_repo = Arc::new(RoleRepositoryImpl::new(RoleSeaOrmRepository::new(db.clone())));
    let role_service = Arc::new(RoleService::new(role_repo.clone())) as Arc<dyn RolePort>;

    // Permission wiring
    let permission_repo = Arc::new(PermissionRepositoryImpl::new(PermissionSeaOrmRepository::new(db.clone())));
    let permission_service = Arc::new(PermissionService::new(permission_repo.clone())) as Arc<dyn PermissionPort>;

    // Auth wiring
    let refresh_token_repo = Arc::new(RefreshTokenRepositoryImpl::new(RefreshTokenSeaOrmRepository::new(db.clone())));
    let auth_service = Arc::new(AuthService::new(
        user_repo,
        tenant_repo,
        refresh_token_repo,
    )) as Arc<dyn AuthPort>;

    Arc::new(AppState {
        tenant_service,
        user_service,
        role_service,
        permission_service,
        auth_service,
        enforcer,
    })
}
