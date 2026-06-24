use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use crawler_domain::dto::User::User;
use crawler_domain::port::output::UserRepositoryPort::UserRepositoryPort;
use crawler_domain::r#enum::DomainError::DomainError;

use crate::user::mapper::UserDataMapper::UserDataMapper;
use crate::user::repository::UserSeaOrmRepository::UserSeaOrmRepository;

pub struct UserRepositoryImpl {
    sea_orm_repo: UserSeaOrmRepository,
}

impl UserRepositoryImpl {
    pub fn new(sea_orm_repo: UserSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl UserRepositoryPort for UserRepositoryImpl {
    async fn save(&self, user: &User) -> Result<(), DomainError> {
        let active_model = UserDataMapper::to_active_model(user);
        self.sea_orm_repo.insert(active_model).await.map_err(|e| {
            error!(error = %e, "Failed to save user via SeaORM");
            DomainError::DownloadFailed(format!("Failed to save user: {}", e))
        })?;
        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), DomainError> {
        let active_model = UserDataMapper::to_active_model(user);
        self.sea_orm_repo.update(active_model).await.map_err(|e| {
            error!(error = %e, "Failed to update user via SeaORM");
            DomainError::DownloadFailed(format!("Failed to update user: {}", e))
        })?;
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let model = self.sea_orm_repo.find_by_id(id).await.map_err(|e| {
            error!(error = %e, "Failed to find user via SeaORM");
            DomainError::DownloadFailed(format!("Failed to query user: {}", e))
        })?;
        Ok(model.as_ref().map(UserDataMapper::to_domain))
    }

    async fn find_all(&self) -> Result<Vec<User>, DomainError> {
        let models = self.sea_orm_repo.find_all().await.map_err(|e| {
            error!(error = %e, "Failed to list users via SeaORM");
            DomainError::DownloadFailed(format!("Failed to list users: {}", e))
        })?;
        Ok(models.iter().map(UserDataMapper::to_domain).collect())
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await.map_err(|e| {
            error!(error = %e, "Failed to delete user via SeaORM");
            DomainError::DownloadFailed(format!("Failed to delete user: {}", e))
        })
    }
}
