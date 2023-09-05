use anyhow::Result;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::Message;

pub trait PromptValue {
    fn to_string(&self) -> String;
    fn to_messages(&self) -> Vec<Message>;
}

#[async_trait::async_trait]
pub trait LLM: Send + Sync {
    async fn generate(&self, text: &str) -> Result<GenerateResult>;
    // async fn predict_messages(&self, messages: Vec<Message>, stop: Option<&str>) -> Message;
    fn tokenize(&self, text: &str) -> Result<Vec<String>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateResult {
    pub tokens: Option<TokenUsage>,
    pub generation: String,
}

impl Default for GenerateResult {
    fn default() -> Self {
        Self {
            tokens: Default::default(),
            generation: Default::default(),
        }
    }
}
