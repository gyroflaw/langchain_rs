use anyhow::Result;

use super::Message;

#[async_trait::async_trait]
pub trait MessageHistory: Send + Sync {
    async fn add_message(&self, message: Message) -> Result<()>;
    async fn add_messages(&self, messages: Vec<Message>) -> Result<()>;
    async fn get_messages(&self, page: usize, page_size: usize) -> Result<Vec<Message>>;
    async fn clear(&self) -> Result<()>;
}
