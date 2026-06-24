use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::CreatePermissionCommand::CreatePermissionCommand;
use crate::dto::Permission::Permission;
use crate::dto::PermissionResponse::PermissionResponse;
use crate::dto::UpdatePermissionCommand::UpdatePermissionCommand;
use crate::port::input::PermissionPort::PermissionPort;
use crate::port::output::PermissionRepositoryPort::PermissionRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct PermissionService {
    permission_repository: Arc<dyn PermissionRepositoryPort>,
}

impl PermissionService {
    pub fn new(permission_repository: Arc<dyn PermissionRepositoryPort>) -> Self {
        Self { permission_repository }
    }

    fn to_response(p: &Permission) -> PermissionResponse {
        PermissionResponse {
            id: p.id,
            name: p.name.clone(),
            action: p.action.clone(),
            resource: p.resource.clone(),
            description: p.description.clone(),
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}

#[async_trait]
impl PermissionPort for PermissionService {
    async fn create_permission(&self, command: CreatePermissionCommand) -> Result<PermissionResponse, DomainError> {
        info!(name = %command.name, "Creating permission");
        if self.permission_repository.find_by_name(&command.name).await?.is_some() {
            return Err(DomainError::AlreadyExists(format!("Permission '{}' already exists", command.name)));
        }
        let permission = Permission::new(command.name, command.action, command.resource, command.description);
        self.permission_repository.save(&permission).await?;
        info!(permission_id = %permission.id, "Permission created");
        Ok(Self::to_response(&permission))
    }

    async fn find_permission_by_id(&self, id: Uuid) -> Result<PermissionResponse, DomainError> {
        let p = self.permission_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Permission not found: {}", id)))?;
        Ok(Self::to_response(&p))
    }

    async fn find_all_permissions(&self) -> Result<Vec<PermissionResponse>, DomainError> {
        let perms = self.permission_repository.find_all().await?;
        Ok(perms.iter().map(Self::to_response).collect())
    }

    async fn update_permission(&self, id: Uuid, command: UpdatePermissionCommand) -> Result<PermissionResponse, DomainError> {
        info!(permission_id = %id, "Updating permission");
        let mut p = self.permission_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Permission not found: {}", id)))?;
        if let Some(name) = command.name {
            p.name = name;
        }
        if let Some(desc) = command.description {
            p.description = Some(desc);
        }
        p.updated_at = Utc::now();
        self.permission_repository.update(&p).await?;
        Ok(Self::to_response(&p))
    }

    async fn delete_permission(&self, id: Uuid) -> Result<(), DomainError> {
        info!(permission_id = %id, "Deleting permission");
        let deleted = self.permission_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(permission_id = %id, "Permission not found for deletion");
            return Err(DomainError::NotFound(format!("Permission not found: {}", id)));
        }
        Ok(())
    }
}
