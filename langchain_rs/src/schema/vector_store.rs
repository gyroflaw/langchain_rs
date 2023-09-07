use anyhow::Result;

use super::Document;
use super::Documents;

#[async_trait::async_trait]
pub trait VectorStore: Send + Sync {
    /// Initialize connection to the vectorstore.
    async fn init(&self) -> Result<()>;

    /// Run more texts through the embeddings and add to the vectorstore.
    /// # Arguments
    /// * `texts` - texts to add the vectorstore
    /// # Returns
    /// List of ids from adding the texts into the vectorstore.
    async fn add_texts(&self, texts: Vec<String>) -> Result<Vec<String>>;

    /// Run more documents through the embeddings and add to the vectorstore.
    /// # Arguments
    /// * `documents` - documents to add the vectorstore
    /// # Returns
    /// List of IDs of the added documents
    async fn add_documents(&self, documents: &Documents) -> Result<Vec<String>>;

    /// Delete by vector ID
    /// # Returns
    /// Ok if deleted successfully, Err otherwise
    async fn delete(&self, ids: Vec<String>) -> Result<()>;

    /// Return docs most similar to query.
    /// # Arguments
    /// * `k`: Number of Documents to return.
    /// # Returns
    /// List of Documents selected by similarity.
    async fn similarity_search(&self, query: &str, k: usize) -> Result<Vec<Document>>;

    /// Return docs selected using the maximal marginal relevance.
    /// Maximal marginal relevance optimizes for similarity to query AND diversity
    /// among selected documents.
    /// # Arguments
    /// * `query`: Text to look up documents similar to.
    /// * `k`: Number of Documents to return.
    /// * `fetch_k`: Number of Documents to fetch to pass to MMR algorithm.
    /// * `lambda_mult`: Number between 0 and 1 that determines the degree
    /// of diversity among the results with 0 corresponding
    /// to maximum diversity and 1 to minimum diversity.
    /// # Returns
    /// List of Documents selected by maximal marginal relevance.
    async fn max_marginal_relevance_search(
        &self,
        query: &str,
        k: usize,
        fetch_k: usize,
        lambda_mult: f32,
    ) -> Result<Vec<Document>>;
}
