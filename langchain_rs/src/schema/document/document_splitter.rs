use anyhow::Result;

use super::documents::Documents;

pub trait DocumentSplitter {
    fn separators(&self) -> Vec<String>;
    fn split_documents(&self, documents: &Documents) -> Result<Documents>;
}
