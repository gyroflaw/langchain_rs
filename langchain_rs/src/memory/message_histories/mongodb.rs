use anyhow::Result;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::FindOptions;

use crate::schema::Message;
use crate::schema::MessageHistory;

#[allow(dead_code)]
pub struct MongoDBMessageHistory {
    client: mongodb::Client,
    db: mongodb::Database,
    collection: mongodb::Collection<Message>,
}

impl MongoDBMessageHistory {
    pub async fn new(
        mongodb_url: Option<&str>,
        db: Option<&str>,
        collection: Option<&str>,
    ) -> Self {
        let client =
            mongodb::Client::with_uri_str(mongodb_url.unwrap_or("mongodb://localhost:27017/"))
                .await
                .unwrap();
        let db = client.database(db.unwrap_or("langchain"));
        let collection = db.collection(collection.unwrap_or("message_history"));
        Self {
            client,
            db,
            collection,
        }
    }
}

#[async_trait::async_trait]
impl MessageHistory for MongoDBMessageHistory {
    async fn init(&self) -> Result<()> {
        unimplemented!("Not needed to implement")
    }

    async fn add_message(&self, message: Message) -> Result<()> {
        self.collection.insert_one(message, None).await?;
        Ok(())
    }

    async fn add_messages(&self, messages: Vec<Message>) -> Result<()> {
        self.collection.insert_many(messages, None).await?;
        Ok(())
    }

    async fn get_messages(&self, _page: usize, _page_size: usize) -> Result<Vec<Message>> {
        let find_options = FindOptions::builder().skip(0).limit(20).build();
        let mut cursor = self.collection.find(None, find_options).await?;

        let mut messages = Vec::new();

        while let Some(message) = cursor.try_next().await? {
            messages.push(message);
        }

        Ok(messages)
    }

    async fn clear(&self) -> Result<()> {
        self.collection.delete_many(doc! {}, None).await?;
        Ok(())
    }
}
