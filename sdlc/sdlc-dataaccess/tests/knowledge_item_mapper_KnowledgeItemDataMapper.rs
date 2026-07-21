use sea_orm::Set;
use sdlc_domain::dto::KnowledgeItem::KnowledgeItem;
use sdlc_dataaccess::knowledge_item::entity::KnowledgeItemEntity;
use sdlc_dataaccess::knowledge_item::repository::KnowledgeItemSeaOrmRepository::KnowledgeItemRow;
use sdlc_dataaccess::knowledge_item::mapper::KnowledgeItemDataMapper::KnowledgeItemDataMapper;

#[test]
fn vector_literal_round_trips() {
    let original = vec![0.1_f32, -0.2, 3.0];
    let literal = KnowledgeItemDataMapper::to_vector_literal(&original);
    assert_eq!(literal, "[0.1,-0.2,3]");
    let parsed = KnowledgeItemDataMapper::parse_vector_literal(&literal);
    assert_eq!(parsed, original);
}
