use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::rest::payload::AuthPayloads::{LoginRequest, RefreshRequest, LogoutRequest};
use crate::rest::payload::TenantPayloads::{CreateTenantRequest, UpdateTenantRequest};
use crate::rest::payload::UserPayloads::{CreateUserRequest, UpdateUserRequest};
use crate::rest::payload::RolePayloads::{CreateRoleRequest, UpdateRoleRequest};
use crate::rest::payload::PermissionPayloads::{CreatePermissionRequest, UpdatePermissionRequest};
use crate::rest::response::ApiResponse::ApiResponse;

use sso_domain::dto::TenantResponse::TenantResponse;
use sso_domain::dto::UserResponse::UserResponse;
use sso_domain::dto::RoleResponse::RoleResponse;
use sso_domain::dto::PermissionResponse::PermissionResponse;
use sso_domain::dto::TokenResponse::TokenResponse;

use crate::rest::controller::{
    AuthController, PermissionController, RoleController, TenantController, UserController,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        AuthController::login,
        AuthController::refresh,
        AuthController::logout,
        TenantController::create_tenant,
        TenantController::get_all_tenants,
        TenantController::get_tenant,
        TenantController::update_tenant,
        TenantController::delete_tenant,
        UserController::create_user,
        UserController::get_users_by_tenant,
        UserController::get_user,
        UserController::update_user,
        UserController::delete_user,
        UserController::assign_role,
        UserController::remove_role,
        RoleController::create_role,
        RoleController::get_roles_by_tenant,
        RoleController::get_role,
        RoleController::update_role,
        RoleController::delete_role,
        RoleController::assign_permission,
        RoleController::remove_permission,
        PermissionController::create_permission,
        PermissionController::get_all_permissions,
        PermissionController::get_permission,
        PermissionController::update_permission,
        PermissionController::delete_permission,
    ),
    components(
        schemas(
            LoginRequest,
            RefreshRequest,
            LogoutRequest,
            CreateTenantRequest,
            UpdateTenantRequest,
            CreateUserRequest,
            UpdateUserRequest,
            CreateRoleRequest,
            UpdateRoleRequest,
            CreatePermissionRequest,
            UpdatePermissionRequest,
            ApiResponse<TenantResponse>,
            ApiResponse<UserResponse>,
            ApiResponse<RoleResponse>,
            ApiResponse<PermissionResponse>,
            ApiResponse<TokenResponse>,
            ApiResponse<Vec<TenantResponse>>,
            ApiResponse<Vec<UserResponse>>,
            ApiResponse<Vec<RoleResponse>>,
            ApiResponse<Vec<PermissionResponse>>,
            TenantResponse,
            UserResponse,
            RoleResponse,
            PermissionResponse,
            TokenResponse,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Auth", description = "Authentication APIs"),
        (name = "Tenants", description = "Tenant management APIs"),
        (name = "Users", description = "User management APIs"),
        (name = "Roles", description = "Role management APIs"),
        (name = "Permissions", description = "Permission management APIs"),
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
