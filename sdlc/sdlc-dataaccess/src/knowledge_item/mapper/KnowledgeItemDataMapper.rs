use sea_orm::Set;

use sdlc_domain::dto::KnowledgeItem::KnowledgeItem;

use crate::knowledge_item::entity::KnowledgeItemEntity;
use crate::knowledge_item::repository::KnowledgeItemSeaOrmRepository::KnowledgeItemRow;

pub struct KnowledgeItemDataMapper;

impl KnowledgeItemDataMapper {
    pub fn to_domain(row: &KnowledgeItemRow) -> KnowledgeItem {
        KnowledgeItem {
            id: row.id,
            tenant_id: row.tenant_id,
            source_type: row.source_type.clone(),
            key: row.key.clone(),
            version: row.version,
            title: row.title.clone(),
            content: row.content.clone(),
            metadata: row.metadata.clone(),
            embedding: row.embedding.as_deref().map(Self::parse_vector_literal),
            is_active: row.is_active,
            created_at: row.created_at,
            updated_at: row.updated_at,
            created_by: row.created_by,
            updated_by: row.updated_by,
        }
    }

    pub fn to_active_model(item: &KnowledgeItem) -> KnowledgeItemEntity::ActiveModel {
        KnowledgeItemEntity::ActiveModel {
            id: Set(item.id),
            tenant_id: Set(item.tenant_id),
            source_type: Set(item.source_type.clone()),
            key: Set(item.key.clone()),
            version: Set(item.version),
            title: Set(item.title.clone()),
            content: Set(item.content.clone()),
            metadata: Set(item.metadata.clone()),
            is_active: Set(item.is_active),
            created_at: Set(item.created_at),
            updated_at: Set(item.updated_at),
            created_by: Set(item.created_by),
            updated_by: Set(item.updated_by),
        }
    }

    /// pgvector's textual input/output format: `[0.1,-0.2,0.3]`.
    pub fn to_vector_literal(embedding: &[f32]) -> String {
        let mut literal = String::from("[");
        for (i, value) in embedding.iter().enumerate() {
            if i > 0 {
                literal.push(',');
            }
            literal.push_str(&value.to_string());
        }
        literal.push(']');
        literal
    }

    pub fn parse_vector_literal(text: &str) -> Vec<f32> {
        text.trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .filter_map(|s| s.trim().parse::<f32>().ok())
            .collect()
    }
}
