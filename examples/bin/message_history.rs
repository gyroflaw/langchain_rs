use anyhow::Result;
use langchain_rs::memory::MongoDBMessageHistory;
use langchain_rs::schema::Message;
use langchain_rs::schema::MessageHistory;
use langchain_rs::schema::MessageType;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let mongodb_message = MongoDBMessageHistory::new(None, None, None).await;

    mongodb_message
        .add_message(Message {
            content: "Hello".to_string(),
            message_type: MessageType::AIMessage,
        })
        .await?;

    let messages = mongodb_message.get_messages(0, 20).await?;
    print!("{:#?}", messages);
    Ok(())
}
