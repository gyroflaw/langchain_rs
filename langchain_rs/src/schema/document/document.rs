use crate::utils::split_by_token;
use crate::utils::TokenizerModel;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Document {
    pub path: String,
    pub content: String,
}

impl Document {
    pub fn new(path: &str, content: &str) -> Self {
        Document {
            path: path.to_string(),
            content: content.to_string(),
        }
    }

    pub fn content_md5(&self) -> String {
        format!("{:x}", md5::compute(self.content.clone()))
    }

    pub fn tokens(&self, model: Option<TokenizerModel>) -> usize {
        split_by_token(&self.content, model).unwrap().len()
    }

    pub fn size(&self) -> usize {
        self.content.len()
    }
}
