#![allow(non_snake_case)]

// Domain models
pub mod Tenant;
pub mod User;
pub mod Role;
pub mod Permission;
pub mod UserRole;
pub mod RolePermission;
pub mod RefreshToken;

// Tenant DTOs
pub mod CreateTenantCommand;
pub mod UpdateTenantCommand;
pub mod TenantResponse;

// User DTOs
pub mod CreateUserCommand;
pub mod UpdateUserCommand;
pub mod UserResponse;

// Role DTOs
pub mod CreateRoleCommand;
pub mod UpdateRoleCommand;
pub mod RoleResponse;

// Permission DTOs
pub mod CreatePermissionCommand;
pub mod UpdatePermissionCommand;
pub mod PermissionResponse;

// Auth DTOs
pub mod LoginCommand;
pub mod TokenResponse;
pub mod Claims;
