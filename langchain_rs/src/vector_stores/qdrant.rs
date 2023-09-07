use anyhow::Result;

use crate::schema::Document;
use crate::schema::Documents;
use crate::schema::VectorStore;
pub struct QdrantVectorStore {
    pub url: String,
    pub collection: String,
}

#[async_trait::async_trait]
impl VectorStore for QdrantVectorStore {
    async fn add_documents(&self, inputs: &Documents) -> Result<Vec<String>> {
        todo!()
    }
    async fn similarity_search(&self, query: &str, k: usize) -> Result<Vec<Document>> {
        todo!()
    }
}
