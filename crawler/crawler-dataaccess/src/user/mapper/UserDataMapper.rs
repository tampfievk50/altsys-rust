use sea_orm::Set;

use crawler_domain::dto::User::User;

use crate::user::entity::UserEntity;

pub struct UserDataMapper;

impl UserDataMapper {
    pub fn to_domain(model: &UserEntity::Model) -> User {
        User {
            id: model.id,
            username: model.username.clone(),
            email: model.email.clone(),
            password_hash: model.password_hash.clone(),
            full_name: model.full_name.clone(),
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
        }
    }

    pub fn to_active_model(user: &User) -> UserEntity::ActiveModel {
        UserEntity::ActiveModel {
            id: Set(user.id),
            username: Set(user.username.clone()),
            email: Set(user.email.clone()),
            password_hash: Set(user.password_hash.clone()),
            full_name: Set(user.full_name.clone()),
            is_active: Set(user.is_active),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
            created_by: Set(user.created_by),
            updated_by: Set(user.updated_by),
        }
    }
}
