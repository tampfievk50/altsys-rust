use sea_orm::Set;

use sdlc_domain::dto::Credential::Credential;

use crate::credential::entity::CredentialEntity;

pub struct CredentialDataMapper;

impl CredentialDataMapper {
    pub fn to_domain(model: &CredentialEntity::Model) -> Credential {
        Credential {
            id: model.id,
            tenant_id: model.tenant_id,
            name: model.name.clone(),
            provider: model.provider.clone(),
            encrypted_secret: model.encrypted_secret.clone(),
            secret_hint: model.secret_hint.clone(),
            metadata: model.metadata.clone(),
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
        }
    }

    pub fn to_active_model(credential: &Credential) -> CredentialEntity::ActiveModel {
        CredentialEntity::ActiveModel {
            id: Set(credential.id),
            tenant_id: Set(credential.tenant_id),
            name: Set(credential.name.clone()),
            provider: Set(credential.provider.clone()),
            encrypted_secret: Set(credential.encrypted_secret.clone()),
            secret_hint: Set(credential.secret_hint.clone()),
            metadata: Set(credential.metadata.clone()),
            is_active: Set(credential.is_active),
            created_at: Set(credential.created_at),
            updated_at: Set(credential.updated_at),
            created_by: Set(credential.created_by),
            updated_by: Set(credential.updated_by),
        }
    }
}
