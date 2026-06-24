use std::sync::Arc;
use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};

use crate::state::AppState::AppState;
use crate::middleware::AuthMiddleware::require_auth;
use crate::middleware::CasbinMiddleware::require_permission;
use crate::rest::controller::{
    AuthController, PermissionController, RoleController, TenantController, UserController,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    // Management routes require authentication
    let management_routes = Router::new()
        // Tenants
        .route("/tenants", get(TenantController::get_all_tenants).post(TenantController::create_tenant))
        .route("/tenants/{id}", get(TenantController::get_tenant).put(TenantController::update_tenant).delete(TenantController::delete_tenant))
        
        // Users
        .route("/users", post(UserController::create_user))
        .route("/tenants/{tenant_id}/users", get(UserController::get_users_by_tenant))
        .route("/users/{id}", get(UserController::get_user).put(UserController::update_user).delete(UserController::delete_user))
        .route("/users/{id}/roles/{role_id}/{tenant_id}", post(UserController::assign_role))
        .route("/users/{id}/roles/{role_id}", delete(UserController::remove_role))
        
        // Roles
        .route("/roles", post(RoleController::create_role))
        .route("/tenants/{tenant_id}/roles", get(RoleController::get_roles_by_tenant))
        .route("/roles/{id}", get(RoleController::get_role).put(RoleController::update_role).delete(RoleController::delete_role))
        .route("/roles/{id}/permissions/{permission_id}", post(RoleController::assign_permission).delete(RoleController::remove_permission))
        
        // Permissions
        .route("/permissions", get(PermissionController::get_all_permissions).post(PermissionController::create_permission))
        .route("/permissions/{id}", get(PermissionController::get_permission).put(PermissionController::update_permission).delete(PermissionController::delete_permission))
        
        .layer(middleware::from_fn_with_state(state.clone(), require_permission))
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    // Auth routes are public (logout uses token in body for simplicity, though could require auth)
    let auth_routes = Router::new()
        .route("/auth/login", post(AuthController::login))
        .route("/auth/refresh", post(AuthController::refresh))
        .route("/auth/logout", post(AuthController::logout));

    Router::new()
        .nest("/api/v1", management_routes)
        .nest("/api/v1", auth_routes)
        .with_state(state)
}
