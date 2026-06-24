use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::CreateTenantCommand::CreateTenantCommand;
use crate::dto::UpdateTenantCommand::UpdateTenantCommand;
use crate::dto::TenantResponse::TenantResponse;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait TenantPort: Send + Sync {
    async fn create_tenant(&self, command: CreateTenantCommand) -> Result<TenantResponse, DomainError>;
    async fn find_tenant_by_id(&self, id: Uuid) -> Result<TenantResponse, DomainError>;
    async fn find_tenant_by_slug(&self, slug: &str) -> Result<TenantResponse, DomainError>;
    async fn find_all_tenants(&self) -> Result<Vec<TenantResponse>, DomainError>;
    async fn update_tenant(&self, id: Uuid, command: UpdateTenantCommand) -> Result<TenantResponse, DomainError>;
    async fn delete_tenant(&self, id: Uuid) -> Result<(), DomainError>;
}
