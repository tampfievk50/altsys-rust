use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sdlc_domain::dto::CreateKnowledgeItemCommand::CreateKnowledgeItemCommand;
use sdlc_domain::dto::UpdateKnowledgeItemCommand::UpdateKnowledgeItemCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateKnowledgeItemRequest {
    pub tenant_id: Uuid,
    pub source_type: String,
    pub key: String,
    pub title: String,
    pub content: String,
    pub metadata: Option<String>,
}

impl From<CreateKnowledgeItemRequest> for CreateKnowledgeItemCommand {
    fn from(val: CreateKnowledgeItemRequest) -> Self {
        CreateKnowledgeItemCommand {
            tenant_id: val.tenant_id,
            source_type: val.source_type,
            key: val.key,
            title: val.title,
            content: val.content,
            metadata: val.metadata,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateKnowledgeItemRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub metadata: Option<String>,
    pub is_active: Option<bool>,
}

impl From<UpdateKnowledgeItemRequest> for UpdateKnowledgeItemCommand {
    fn from(val: UpdateKnowledgeItemRequest) -> Self {
        UpdateKnowledgeItemCommand {
            title: val.title,
            content: val.content,
            metadata: val.metadata,
            is_active: val.is_active,
        }
    }
}
