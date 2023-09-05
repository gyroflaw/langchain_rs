use anyhow::Result;
use tiktoken_rs::cl100k_base;
use tiktoken_rs::p50k_base;
use tiktoken_rs::p50k_edit;
use tiktoken_rs::r50k_base;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TokenizerModel {
    R50kBase,
    P50kBase,
    P50kEdit,
    Cl100kBase,
}

pub fn split_by_token(input: &str, model: Option<TokenizerModel>) -> Result<Vec<String>> {
    let rke = match model.unwrap_or(TokenizerModel::R50kBase) {
        TokenizerModel::R50kBase => r50k_base()?,
        TokenizerModel::P50kBase => p50k_base()?,
        TokenizerModel::P50kEdit => p50k_edit()?,
        TokenizerModel::Cl100kBase => cl100k_base()?,
    };
    rke.split_by_token(input, true)
}
