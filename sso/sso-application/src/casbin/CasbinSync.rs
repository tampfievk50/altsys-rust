use casbin::{Enforcer, MgmtApi};

use sso_domain::r#enum::DomainError::DomainError;

use crate::exception::GlobalExceptionHandler::AppError;

/// Keeps the enforcer's `p`/`g` rows (and therefore the `casbin_rule` table,
/// via the SeaOrm adapter) in sync with role/permission and user/role
/// assignments made through the Roles and Users APIs. Without this, the
/// domain tables (role_permissions, user_roles) and Casbin's own rule table
/// silently drift apart — enforcement would keep using whatever was last
/// synced instead of the current assignments.
fn sync_err(err: casbin::Error) -> AppError {
    AppError(DomainError::InternalError(format!("Casbin sync failed: {err}")))
}

pub async fn grant_permission(enforcer: &mut Enforcer, role_name: &str, resource: &str, action: &str) -> Result<(), AppError> {
    enforcer
        .add_policy(vec![role_name.to_string(), resource.to_string(), action.to_string()])
        .await
        .map_err(sync_err)?;
    Ok(())
}

pub async fn revoke_permission(enforcer: &mut Enforcer, role_name: &str, resource: &str, action: &str) -> Result<(), AppError> {
    enforcer
        .remove_policy(vec![role_name.to_string(), resource.to_string(), action.to_string()])
        .await
        .map_err(sync_err)?;
    Ok(())
}

pub async fn grant_role(enforcer: &mut Enforcer, user_id: &str, role_name: &str) -> Result<(), AppError> {
    enforcer
        .add_grouping_policy(vec![user_id.to_string(), role_name.to_string()])
        .await
        .map_err(sync_err)?;
    Ok(())
}

pub async fn revoke_role(enforcer: &mut Enforcer, user_id: &str, role_name: &str) -> Result<(), AppError> {
    enforcer
        .remove_grouping_policy(vec![user_id.to_string(), role_name.to_string()])
        .await
        .map_err(sync_err)?;
    Ok(())
}

/// Cascade cleanup for a deleted role: drops every `p` row where it's the
/// subject and every `g` row that groups a user into it, regardless of
/// which permissions/users those were. Also used per-role when a tenant is
/// deleted, since deleting the tenant alone doesn't tell us which p/g rows
/// its roles left behind.
pub async fn revoke_all_for_role(enforcer: &mut Enforcer, role_name: &str) -> Result<(), AppError> {
    enforcer
        .remove_filtered_policy(0, vec![role_name.to_string()])
        .await
        .map_err(sync_err)?;
    enforcer
        .remove_filtered_grouping_policy(1, vec![role_name.to_string()])
        .await
        .map_err(sync_err)?;
    Ok(())
}

/// Cascade cleanup for a deleted user: drops every `g` row for that user id
/// regardless of role. Also used per-user when a tenant is deleted.
pub async fn revoke_all_for_user(enforcer: &mut Enforcer, user_id: &str) -> Result<(), AppError> {
    enforcer
        .remove_filtered_grouping_policy(0, vec![user_id.to_string()])
        .await
        .map_err(sync_err)?;
    Ok(())
}

/// Cascade cleanup for a deleted permission: drops every `p` row for that
/// resource/action regardless of which role(s) held it.
pub async fn revoke_all_for_permission(enforcer: &mut Enforcer, resource: &str, action: &str) -> Result<(), AppError> {
    enforcer
        .remove_filtered_policy(1, vec![resource.to_string(), action.to_string()])
        .await
        .map_err(sync_err)?;
    Ok(())
}
