use anyhow::Result;
use qdrant_client::prelude::QdrantClient;
use qdrant_client::qdrant::CreateCollection;

use crate::schema::Document;
use crate::schema::Documents;
use crate::schema::VectorStore;

pub struct QdrantVectorStore {
    pub url: String,
    api_key: Option<String>,
    pub collection: String,
    options: Option<CreateCollection>,
}

impl QdrantVectorStore {
    pub fn new(
        url: &str,
        api_key: Option<&str>,
        collection: &str,
        options: Option<CreateCollection>,
    ) -> Self {
        QdrantVectorStore {
            url: url.to_string(),
            api_key: api_key.map(|s| s.to_string()),
            collection: collection.to_string(),
            options,
        }
    }

    pub fn with_url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    pub fn with_collection(mut self, collection: &str) -> Self {
        self.collection = collection.to_string();
        self
    }

    pub fn with_options(mut self, options: CreateCollection) -> Self {
        self.options = Some(options);
        self
    }

    pub fn get_client(&self) -> Result<QdrantClient> {
        let client = QdrantClient::from_url(&self.url)
            .with_api_key(self.api_key.clone())
            .build()?;
        Ok(client)
    }

    pub async fn has_collection(&self) -> Result<bool> {
        let client = self.get_client()?;
        let has_collection = client.has_collection(&self.collection).await?;
        Ok(has_collection)
    }
}

#[async_trait::async_trait]
impl VectorStore for QdrantVectorStore {
    async fn init(&self) -> Result<()> {
        let client = QdrantClient::from_url(&self.url)
            .with_api_key(self.api_key.clone())
            .build()?;

        if !client.has_collection(&self.collection).await? {
            client
                .create_collection(&self.options.clone().unwrap())
                .await?;
        }

        Ok(())
    }
    async fn add_texts(&self, _texts: Vec<String>) -> Result<Vec<String>> {
        todo!()
    }
    async fn add_documents(&self, _documents: &Documents) -> Result<Vec<String>> {
        todo!()
    }

    async fn delete(&self, _ids: Vec<String>) -> Result<()> {
        todo!()
    }

    async fn similarity_search(&self, _query: &str, _k: usize) -> Result<Vec<Document>> {
        todo!()
    }

    async fn max_marginal_relevance_search(
        &self,
        _query: &str,
        _k: usize,
        _fetch_k: usize,
        _lambda_mult: f32,
    ) -> Result<Vec<Document>> {
        todo!()
    }
}
