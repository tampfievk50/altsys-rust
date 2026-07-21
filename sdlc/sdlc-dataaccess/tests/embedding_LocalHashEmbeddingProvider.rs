use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use async_trait::async_trait;
use sdlc_domain::port::output::EmbeddingProviderPort::EmbeddingProviderPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_dataaccess::embedding::LocalHashEmbeddingProvider::LocalHashEmbeddingProvider;

#[tokio::test]
async fn embed_fails_on_empty_text() {
    let provider = LocalHashEmbeddingProvider::default();
    let result = provider.embed("   ").await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn embed_is_deterministic() {
    let provider = LocalHashEmbeddingProvider::default();
    let a = provider.embed("Hexagonal architecture ports and adapters").await.unwrap();
    let b = provider.embed("Hexagonal architecture ports and adapters").await.unwrap();
    assert_eq!(a, b);
}

#[tokio::test]
async fn embed_produces_a_unit_vector() {
    let provider = LocalHashEmbeddingProvider::default();
    let v = provider.embed("some knowledge content to embed").await.unwrap();
    let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    assert!((norm - 1.0).abs() < 1e-5);
}

#[tokio::test]
async fn embed_uses_the_requested_dimensionality() {
    let provider = LocalHashEmbeddingProvider::new(16);
    let v = provider.embed("short text").await.unwrap();
    assert_eq!(v.len(), 16);
}
