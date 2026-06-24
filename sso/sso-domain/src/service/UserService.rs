use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::CreateUserCommand::CreateUserCommand;
use crate::dto::UpdateUserCommand::UpdateUserCommand;
use crate::dto::User::User;
use crate::dto::UserResponse::UserResponse;
use crate::dto::UserRole::UserRole;
use crate::port::input::UserPort::UserPort;
use crate::port::output::UserRepositoryPort::UserRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct UserService {
    user_repository: Arc<dyn UserRepositoryPort>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepositoryPort>) -> Self {
        Self { user_repository }
    }

    fn to_response(user: &User) -> UserResponse {
        UserResponse {
            id: user.id,
            tenant_id: user.tenant_id,
            username: user.username.clone(),
            email: user.email.clone(),
            full_name: user.full_name.clone(),
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
            created_by: user.created_by,
            updated_by: user.updated_by,
        }
    }
}

#[async_trait]
impl UserPort for UserService {
    async fn create_user(&self, command: CreateUserCommand) -> Result<UserResponse, DomainError> {
        info!(username = %command.username, "Creating user");
        if command.username.trim().is_empty() {
            return Err(DomainError::ValidationError("Username cannot be empty".into()));
        }
        if command.email.trim().is_empty() {
            return Err(DomainError::ValidationError("Email cannot be empty".into()));
        }
        if self.user_repository.find_by_username_and_tenant(&command.username, command.tenant_id).await?.is_some() {
            return Err(DomainError::AlreadyExists(format!("Username '{}' already exists in this tenant", command.username)));
        }
        let password_hash = bcrypt::hash(&command.password, bcrypt::DEFAULT_COST)
            .map_err(|e| DomainError::InternalError(format!("Password hashing failed: {}", e)))?;
        let user = User::new(command.tenant_id, command.username, command.email, password_hash, command.full_name);
        self.user_repository.save(&user).await?;
        info!(user_id = %user.id, "User created");
        Ok(Self::to_response(&user))
    }

    async fn find_user_by_id(&self, id: Uuid) -> Result<UserResponse, DomainError> {
        let user = self.user_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("User not found: {}", id)))?;
        Ok(Self::to_response(&user))
    }

    async fn find_users_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<UserResponse>, DomainError> {
        let users = self.user_repository.find_by_tenant(tenant_id).await?;
        Ok(users.iter().map(Self::to_response).collect())
    }

    async fn update_user(&self, id: Uuid, command: UpdateUserCommand) -> Result<UserResponse, DomainError> {
        info!(user_id = %id, "Updating user");
        let mut user = self.user_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("User not found: {}", id)))?;
        if let Some(email) = command.email {
            user.email = email;
        }
        if let Some(password) = command.password {
            user.password_hash = bcrypt::hash(&password, bcrypt::DEFAULT_COST)
                .map_err(|e| DomainError::InternalError(format!("Password hashing failed: {}", e)))?;
        }
        if let Some(full_name) = command.full_name {
            user.full_name = Some(full_name);
        }
        if let Some(is_active) = command.is_active {
            user.is_active = is_active;
        }
        user.updated_at = Utc::now();
        self.user_repository.update(&user).await?;
        Ok(Self::to_response(&user))
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), DomainError> {
        info!(user_id = %id, "Deleting user");
        let deleted = self.user_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(user_id = %id, "User not found for deletion");
            return Err(DomainError::NotFound(format!("User not found: {}", id)));
        }
        Ok(())
    }

    async fn assign_role(&self, user_id: Uuid, role_id: Uuid, tenant_id: Uuid) -> Result<(), DomainError> {
        info!(user_id = %user_id, role_id = %role_id, "Assigning role to user");
        let user_role = UserRole { user_id, role_id, tenant_id };
        self.user_repository.save_user_role(&user_role).await
    }

    async fn remove_role(&self, user_id: Uuid, role_id: Uuid) -> Result<(), DomainError> {
        info!(user_id = %user_id, role_id = %role_id, "Removing role from user");
        let removed = self.user_repository.delete_user_role(user_id, role_id).await?;
        if !removed {
            return Err(DomainError::NotFound("User-role assignment not found".into()));
        }
        Ok(())
    }
}
