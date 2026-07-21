use async_trait::async_trait;

use crate::r#enum::DomainError::DomainError;

/// The embedding pipeline: a driven adapter that turns text into a fixed-size vector.
/// Implementations live in `knowledge-dataaccess/src/embedding/` — the default is a local,
/// dependency-free hashing embedder; swap in an HTTP-backed model provider (OpenAI, a local
/// model server, ...) by implementing this trait and rewiring `AppConfig`.
#[async_trait]
pub trait EmbeddingProviderPort: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, DomainError>;
}
