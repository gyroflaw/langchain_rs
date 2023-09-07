use anyhow::Result;

use super::Message;

#[async_trait::async_trait]
pub trait MessageHistory: Send + Sync {
    /// Initialize connection to the message store
    async fn init(&self) -> Result<()>;
    /// Add a message to the message store.
    /// # Arguments
    /// * `message`: message to add
    async fn add_message(&self, message: Message) -> Result<()>;
    /// Add messages to the message store.
    /// # Arguments
    /// * `messages`: messages to add
    async fn add_messages(&self, messages: Vec<Message>) -> Result<()>;

    /// Retrieve messages from the message store
    /// TODO: Consider `impl Iterator<Item = Message>` for return type
    /// # Arguments
    /// * `page`: page to retrieve
    /// * `page_size`: message count per page
    async fn get_messages(&self, page: usize, page_size: usize) -> Result<Vec<Message>>;

    /// Clear all messages from the mssage store.
    async fn clear(&self) -> Result<()>;
}
