use anyhow::Result;

use super::document_path::DocumentPath;
use super::documents::Documents;

#[async_trait::async_trait]
pub trait DocumentLoader: Send + Sync {
    async fn load(&self, path: DocumentPath) -> Result<Documents>;
}
