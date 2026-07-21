use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use async_trait::async_trait;

use sdlc_domain::port::output::EmbeddingProviderPort::EmbeddingProviderPort;
use sdlc_domain::r#enum::DomainError::DomainError;

/// Default embedding pipeline implementation: a deterministic, dependency-free
/// "hashing trick" bag-of-words embedder (each whitespace token is hashed into one
/// of `dimensions` buckets, then the vector is L2-normalized). It needs no network
/// call or API key, so the knowledge pipeline works fully offline out of the box.
///
/// This is intentionally swappable: implement `EmbeddingProviderPort` against a real
/// model provider (OpenAI-compatible HTTP API, a local model server, ...) and rewire
/// it in `AppConfig` once one is available — the rest of the domain is unaffected.
pub struct LocalHashEmbeddingProvider {
    dimensions: usize,
}

impl LocalHashEmbeddingProvider {
    pub fn new(dimensions: usize) -> Self {
        Self { dimensions }
    }
}

impl Default for LocalHashEmbeddingProvider {
    fn default() -> Self {
        Self::new(256)
    }
}

#[async_trait]
impl EmbeddingProviderPort for LocalHashEmbeddingProvider {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, DomainError> {
        if text.trim().is_empty() {
            return Err(DomainError::ValidationError("Cannot embed empty text".into()));
        }

        let mut vector = vec![0f32; self.dimensions];
        for token in text.split_whitespace() {
            let mut hasher = DefaultHasher::new();
            token.to_lowercase().hash(&mut hasher);
            let bucket = (hasher.finish() as usize) % self.dimensions;
            vector[bucket] += 1.0;
        }

        let norm = vector.iter().map(|v| v * v).sum::<f32>().sqrt();
        if norm > 0.0 {
            for v in vector.iter_mut() {
                *v /= norm;
            }
        }
        Ok(vector)
    }
}
