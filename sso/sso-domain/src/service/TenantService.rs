use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::CreateTenantCommand::CreateTenantCommand;
use crate::dto::Tenant::Tenant;
use crate::dto::TenantResponse::TenantResponse;
use crate::dto::UpdateTenantCommand::UpdateTenantCommand;
use crate::port::input::TenantPort::TenantPort;
use crate::port::output::TenantRepositoryPort::TenantRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct TenantService {
    tenant_repository: Arc<dyn TenantRepositoryPort>,
}

impl TenantService {
    pub fn new(tenant_repository: Arc<dyn TenantRepositoryPort>) -> Self {
        Self { tenant_repository }
    }

    fn to_response(tenant: &Tenant) -> TenantResponse {
        TenantResponse {
            id: tenant.id,
            name: tenant.name.clone(),
            slug: tenant.slug.clone(),
            is_active: tenant.is_active,
            created_at: tenant.created_at,
            updated_at: tenant.updated_at,
        }
    }
}

#[async_trait]
impl TenantPort for TenantService {
    async fn create_tenant(&self, command: CreateTenantCommand) -> Result<TenantResponse, DomainError> {
        info!(slug = %command.slug, "Creating tenant");
        if command.name.trim().is_empty() {
            return Err(DomainError::ValidationError("Tenant name cannot be empty".into()));
        }
        if command.slug.trim().is_empty() {
            return Err(DomainError::ValidationError("Tenant slug cannot be empty".into()));
        }
        if self.tenant_repository.find_by_slug(&command.slug).await?.is_some() {
            return Err(DomainError::AlreadyExists(format!("Tenant slug '{}' already exists", command.slug)));
        }
        let tenant = Tenant::new(command.name, command.slug);
        self.tenant_repository.save(&tenant).await?;
        info!(tenant_id = %tenant.id, "Tenant created");
        Ok(Self::to_response(&tenant))
    }

    async fn find_tenant_by_id(&self, id: Uuid) -> Result<TenantResponse, DomainError> {
        let tenant = self.tenant_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Tenant not found: {}", id)))?;
        Ok(Self::to_response(&tenant))
    }

    async fn find_tenant_by_slug(&self, slug: &str) -> Result<TenantResponse, DomainError> {
        let tenant = self.tenant_repository.find_by_slug(slug).await?
            .ok_or_else(|| DomainError::NotFound(format!("Tenant not found: {}", slug)))?;
        Ok(Self::to_response(&tenant))
    }

    async fn find_all_tenants(&self) -> Result<Vec<TenantResponse>, DomainError> {
        let tenants = self.tenant_repository.find_all().await?;
        Ok(tenants.iter().map(Self::to_response).collect())
    }

    async fn update_tenant(&self, id: Uuid, command: UpdateTenantCommand) -> Result<TenantResponse, DomainError> {
        info!(tenant_id = %id, "Updating tenant");
        let mut tenant = self.tenant_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Tenant not found: {}", id)))?;
        if let Some(name) = command.name {
            tenant.name = name;
        }
        if let Some(is_active) = command.is_active {
            tenant.is_active = is_active;
        }
        tenant.updated_at = Utc::now();
        self.tenant_repository.update(&tenant).await?;
        Ok(Self::to_response(&tenant))
    }

    async fn delete_tenant(&self, id: Uuid) -> Result<(), DomainError> {
        info!(tenant_id = %id, "Deleting tenant");
        let deleted = self.tenant_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(tenant_id = %id, "Tenant not found for deletion");
            return Err(DomainError::NotFound(format!("Tenant not found: {}", id)));
        }
        Ok(())
    }
}
