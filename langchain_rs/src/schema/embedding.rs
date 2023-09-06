use anyhow::Result;

use super::Documents;

#[async_trait::async_trait]
pub trait Embedding: Send + Sync {
    /// Embed query text.
    /// # Arguments
    /// * `query`: query to embed
    async fn embed_query(&self, query: &str) -> Result<Vec<f32>>;
    /// Embed search docs.
    /// # Arguments
    /// * `documents`: documents to embed
    async fn embed_documents(&self, documents: &Documents) -> Result<Vec<Vec<f32>>>;
}
