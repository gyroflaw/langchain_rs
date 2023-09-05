use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    #[serde(rename = "system")]
    SystemMessage,
    #[serde(rename = "ai")]
    AIMessage,
    #[serde(rename = "human")]
    HumanMessage,
}

impl Default for MessageType {
    fn default() -> Self {
        Self::SystemMessage
    }
}

/// A Message for priming AI behavior, usually passed in as the first of a sequence
/// of input messages.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Message {
    pub content: String,
    pub message_type: MessageType,
}
