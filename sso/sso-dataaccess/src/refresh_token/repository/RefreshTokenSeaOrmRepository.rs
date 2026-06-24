use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use crate::refresh_token::entity::RefreshTokenEntity;

pub struct RefreshTokenSeaOrmRepository {
    db: DatabaseConnection,
}

impl RefreshTokenSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: RefreshTokenEntity::ActiveModel) -> Result<RefreshTokenEntity::Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn find_by_token(&self, token: &str) -> Result<Option<RefreshTokenEntity::Model>, sea_orm::DbErr> {
        RefreshTokenEntity::Entity::find()
            .filter(RefreshTokenEntity::Column::Token.eq(token))
            .one(&self.db)
            .await
    }

    pub async fn revoke(&self, token: &str) -> Result<bool, sea_orm::DbErr> {
        let model = RefreshTokenEntity::Entity::find()
            .filter(RefreshTokenEntity::Column::Token.eq(token))
            .one(&self.db)
            .await?;
        match model {
            Some(m) => {
                let mut active: RefreshTokenEntity::ActiveModel = m.into();
                active.is_revoked = Set(true);
                active.update(&self.db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub async fn revoke_all_for_user(&self, user_id: uuid::Uuid) -> Result<(), sea_orm::DbErr> {
        use sea_orm::ActiveValue::Set as SetVal;
        let models = RefreshTokenEntity::Entity::find()
            .filter(RefreshTokenEntity::Column::UserId.eq(user_id))
            .filter(RefreshTokenEntity::Column::IsRevoked.eq(false))
            .all(&self.db)
            .await?;
        for m in models {
            let mut active: RefreshTokenEntity::ActiveModel = m.into();
            active.is_revoked = SetVal(true);
            active.update(&self.db).await?;
        }
        Ok(())
    }
}
