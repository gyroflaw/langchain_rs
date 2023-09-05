use parking_lot::RwLock;

use super::document::Document;
use crate::utils::TokenizerModel;

#[derive(Debug)]
pub struct Documents {
    documents: RwLock<Vec<Document>>,
}

impl Documents {
    pub fn create() -> Self {
        Documents {
            documents: RwLock::new(vec![]),
        }
    }

    pub fn push(&self, document: Document) {
        self.documents.write().push(document);
    }

    pub fn extend(&self, other_docs: &Documents) {
        self.documents
            .write()
            .extend_from_slice(&other_docs.documents.read());
    }

    pub fn tokens(&self, model: Option<TokenizerModel>) -> usize {
        self.documents.read().iter().map(|d| d.tokens(model)).sum()
    }

    pub fn size(&self) -> usize {
        self.documents.read().iter().map(|d| d.size()).sum()
    }

    pub fn len(&self) -> usize {
        self.documents.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.documents.read().is_empty()
    }

    pub fn iter(&self) -> DocumentsIter {
        let guard = self.documents.read().clone();
        DocumentsIter {
            documents: guard,
            index: 0,
        }
    }

    pub fn first(&self) -> Option<Document> {
        let guard = self.documents.read();
        guard.first().cloned()
    }
}

impl FromIterator<Document> for Documents {
    fn from_iter<I: IntoIterator<Item = Document>>(iter: I) -> Self {
        Documents {
            documents: RwLock::new(iter.into_iter().collect()),
        }
    }
}

impl<'a> IntoIterator for &'a Documents {
    type Item = Document;
    type IntoIter = DocumentsIter;

    fn into_iter(self) -> Self::IntoIter {
        DocumentsIter {
            documents: self.documents.read().clone(),
            index: 0,
        }
    }
}

pub struct DocumentsIter {
    documents: Vec<Document>,
    index: usize,
}

impl Iterator for DocumentsIter {
    type Item = Document;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.documents.len() {
            let result = self.documents[self.index].clone();
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl From<Vec<Document>> for Documents {
    fn from(documents: Vec<Document>) -> Self {
        Documents {
            documents: RwLock::new(documents),
        }
    }
}
