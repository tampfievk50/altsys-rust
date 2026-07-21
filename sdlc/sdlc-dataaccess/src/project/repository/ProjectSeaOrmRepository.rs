use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

use crate::project::entity::ProjectEntity;
use crate::project::entity::ProjectEntity::Model;

pub struct ProjectSeaOrmRepository {
    db: DatabaseConnection,
}

impl ProjectSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: ProjectEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: ProjectEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        ProjectEntity::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Model>, sea_orm::DbErr> {
        ProjectEntity::Entity::find()
            .filter(ProjectEntity::Column::TenantId.eq(tenant_id))
            .all(&self.db)
            .await
    }

    pub async fn find_by_slug_and_tenant(&self, slug: &str, tenant_id: Uuid) -> Result<Option<Model>, sea_orm::DbErr> {
        ProjectEntity::Entity::find()
            .filter(ProjectEntity::Column::Slug.eq(slug))
            .filter(ProjectEntity::Column::TenantId.eq(tenant_id))
            .one(&self.db)
            .await
    }

    pub async fn find_all_with_jira_tool(&self) -> Result<Vec<Model>, sea_orm::DbErr> {
        ProjectEntity::Entity::find()
            .filter(ProjectEntity::Column::JiraToolId.is_not_null())
            .all(&self.db)
            .await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match ProjectEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(model) => { model.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }
}
