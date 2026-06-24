use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::CreateUserCommand::CreateUserCommand;
use crate::dto::UpdateUserCommand::UpdateUserCommand;
use crate::dto::User::User;
use crate::dto::UserResponse::UserResponse;
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
        info!(username = %command.username, "Creating new user");

        if command.username.trim().is_empty() {
            return Err(DomainError::InvalidUrl("Username cannot be empty".to_string()));
        }
        if command.email.trim().is_empty() {
            return Err(DomainError::InvalidUrl("Email cannot be empty".to_string()));
        }

        let user = User::new(
            command.username,
            command.email,
            command.password,
            command.full_name,
        );

        self.user_repository.save(&user).await?;
        info!(user_id = %user.id, "User created successfully");

        Ok(Self::to_response(&user))
    }

    async fn find_user_by_id(&self, id: Uuid) -> Result<UserResponse, DomainError> {
        info!(user_id = %id, "Finding user by ID");

        let user = self
            .user_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("User not found with id: {}", id)))?;

        Ok(Self::to_response(&user))
    }

    async fn find_all_users(&self) -> Result<Vec<UserResponse>, DomainError> {
        info!("Listing all users");

        let users = self.user_repository.find_all().await?;
        Ok(users.iter().map(Self::to_response).collect())
    }

    async fn update_user(&self, id: Uuid, command: UpdateUserCommand) -> Result<UserResponse, DomainError> {
        info!(user_id = %id, "Updating user");

        let mut user = self
            .user_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("User not found with id: {}", id)))?;

        if let Some(email) = command.email {
            user.email = email;
        }
        if let Some(password) = command.password {
            user.password_hash = password;
        }
        if let Some(full_name) = command.full_name {
            user.full_name = Some(full_name);
        }
        if let Some(is_active) = command.is_active {
            user.is_active = is_active;
        }
        user.updated_at = Utc::now();

        self.user_repository.update(&user).await?;
        info!(user_id = %user.id, "User updated successfully");

        Ok(Self::to_response(&user))
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), DomainError> {
        info!(user_id = %id, "Deleting user");

        let deleted = self.user_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(user_id = %id, "User not found for deletion");
            return Err(DomainError::NotFound(format!("User not found with id: {}", id)));
        }

        info!(user_id = %id, "User deleted successfully");
        Ok(())
    }
}
