use std::collections::HashMap;
use std::sync::Arc;
use axum::{extract::State, Json};
use casbin::{CoreApi, MgmtApi};
use serde::Serialize;
use utoipa::ToSchema;

use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use sso_domain::r#enum::DomainError::DomainError;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CasbinPoliciesResponse {
    /// Raw `p` rules as stored by the adapter: [sub, obj, act]
    pub policies: Vec<Vec<String>>,
    /// Raw `g` rules as stored by the adapter: [user, role]
    pub grouping_policies: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CasbinSyncResponse {
    pub policies_synced: usize,
    pub grouping_policies_synced: usize,
}

#[utoipa::path(
    get,
    path = "/api/v1/casbin/policies",
    tag = "Casbin",
    responses(
        (status = 200, description = "List raw Casbin policy and role-assignment rules", body = ApiResponse<CasbinPoliciesResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn list_policies(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<CasbinPoliciesResponse>> {
    let enforcer = state.enforcer.read().await;
    let response = CasbinPoliciesResponse {
        policies: enforcer.get_policy(),
        grouping_policies: enforcer.get_grouping_policy(),
    };

    Json(ApiResponse::success(response))
}

/// Rebuilds every `p`/`g` row in Casbin from the current
/// role_permissions/user_roles assignments across all tenants and features.
/// This is a full rebuild (existing rows are cleared first), so it also
/// discards any rule that isn't backed by a live assignment.
#[utoipa::path(
    post,
    path = "/api/v1/casbin/sync",
    tag = "Casbin",
    responses(
        (status = 200, description = "Rebuilt Casbin policies from current role/permission and user/role assignments", body = ApiResponse<CasbinSyncResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn sync_policies(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<CasbinSyncResponse>>, AppError> {
    let tenants = state.tenant_service.find_all_tenants().await?;

    let permissions_by_id: HashMap<_, _> = state
        .permission_service
        .find_all_permissions()
        .await?
        .into_iter()
        .map(|p| (p.id, p))
        .collect();

    let mut policies = Vec::new();
    let mut grouping_policies = Vec::new();

    for tenant in &tenants {
        let roles = state.role_service.find_roles_by_tenant(tenant.id).await?;

        for role in &roles {
            let permission_ids = state.role_service.find_permission_ids_by_role(role.id).await?;
            for permission_id in permission_ids {
                if let Some(permission) = permissions_by_id.get(&permission_id) {
                    policies.push(vec![role.name.clone(), permission.resource.clone(), permission.action.clone()]);
                }
            }
        }

        let users = state.user_service.find_users_by_tenant(tenant.id).await?;

        for user in &users {
            let role_ids = state.user_service.find_role_ids_by_user(user.id).await?;
            for role_id in role_ids {
                if let Some(role) = roles.iter().find(|r| r.id == role_id) {
                    grouping_policies.push(vec![user.id.to_string(), role.name.clone()]);
                }
            }
        }
    }

    let mut enforcer = state.enforcer.write().await;
    enforcer.clear_policy().await
        .map_err(|e| AppError(DomainError::InternalError(format!("Casbin sync failed: {e}"))))?;

    if !policies.is_empty() {
        enforcer.add_policies(policies.clone()).await
            .map_err(|e| AppError(DomainError::InternalError(format!("Casbin sync failed: {e}"))))?;
    }
    if !grouping_policies.is_empty() {
        enforcer.add_grouping_policies(grouping_policies.clone()).await
            .map_err(|e| AppError(DomainError::InternalError(format!("Casbin sync failed: {e}"))))?;
    }

    Ok(Json(ApiResponse::success(CasbinSyncResponse {
        policies_synced: policies.len(),
        grouping_policies_synced: grouping_policies.len(),
    })))
}
