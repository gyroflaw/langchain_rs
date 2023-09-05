use anyhow::Result;

use super::Document;
use super::Documents;

#[async_trait::async_trait]
pub trait VectorStore: Send + Sync {
    async fn add_documents(&self, inputs: &Documents) -> Result<Vec<String>>;
    async fn similarity_search(&self, query: &str, k: usize) -> Result<Vec<Document>>;
}
