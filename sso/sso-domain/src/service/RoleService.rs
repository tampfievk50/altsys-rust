use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::CreateRoleCommand::CreateRoleCommand;
use crate::dto::Role::Role;
use crate::dto::RolePermission::RolePermission;
use crate::dto::RoleResponse::RoleResponse;
use crate::dto::UpdateRoleCommand::UpdateRoleCommand;
use crate::port::input::RolePort::RolePort;
use crate::port::output::RoleRepositoryPort::RoleRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct RoleService {
    role_repository: Arc<dyn RoleRepositoryPort>,
}

impl RoleService {
    pub fn new(role_repository: Arc<dyn RoleRepositoryPort>) -> Self {
        Self { role_repository }
    }

    fn to_response(role: &Role) -> RoleResponse {
        RoleResponse {
            id: role.id,
            tenant_id: role.tenant_id,
            name: role.name.clone(),
            description: role.description.clone(),
            created_at: role.created_at,
            updated_at: role.updated_at,
        }
    }
}

#[async_trait]
impl RolePort for RoleService {
    async fn create_role(&self, command: CreateRoleCommand) -> Result<RoleResponse, DomainError> {
        info!(name = %command.name, "Creating role");
        if command.name.trim().is_empty() {
            return Err(DomainError::ValidationError("Role name cannot be empty".into()));
        }
        if self.role_repository.find_by_name_and_tenant(&command.name, command.tenant_id).await?.is_some() {
            return Err(DomainError::AlreadyExists(format!("Role '{}' already exists in this tenant", command.name)));
        }
        let role = Role::new(command.tenant_id, command.name, command.description);
        self.role_repository.save(&role).await?;
        info!(role_id = %role.id, "Role created");
        Ok(Self::to_response(&role))
    }

    async fn find_role_by_id(&self, id: Uuid) -> Result<RoleResponse, DomainError> {
        let role = self.role_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Role not found: {}", id)))?;
        Ok(Self::to_response(&role))
    }

    async fn find_roles_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<RoleResponse>, DomainError> {
        let roles = self.role_repository.find_by_tenant(tenant_id).await?;
        Ok(roles.iter().map(Self::to_response).collect())
    }

    async fn update_role(&self, id: Uuid, command: UpdateRoleCommand) -> Result<RoleResponse, DomainError> {
        info!(role_id = %id, "Updating role");
        let mut role = self.role_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Role not found: {}", id)))?;
        if let Some(name) = command.name {
            role.name = name;
        }
        if let Some(desc) = command.description {
            role.description = Some(desc);
        }
        role.updated_at = Utc::now();
        self.role_repository.update(&role).await?;
        Ok(Self::to_response(&role))
    }

    async fn delete_role(&self, id: Uuid) -> Result<(), DomainError> {
        info!(role_id = %id, "Deleting role");
        let deleted = self.role_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(role_id = %id, "Role not found for deletion");
            return Err(DomainError::NotFound(format!("Role not found: {}", id)));
        }
        Ok(())
    }

    async fn assign_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), DomainError> {
        info!(role_id = %role_id, permission_id = %permission_id, "Assigning permission to role");
        let rp = RolePermission { role_id, permission_id };
        self.role_repository.save_role_permission(&rp).await
    }

    async fn remove_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), DomainError> {
        info!(role_id = %role_id, permission_id = %permission_id, "Removing permission from role");
        let removed = self.role_repository.delete_role_permission(role_id, permission_id).await?;
        if !removed {
            return Err(DomainError::NotFound("Role-permission assignment not found".into()));
        }
        Ok(())
    }
}
