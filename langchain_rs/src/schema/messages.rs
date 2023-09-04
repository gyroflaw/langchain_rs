use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    SystemMessage,
    AIMessage,
    HumanMessage,
}

/// A Message for priming AI behavior, usually passed in as the first of a sequence
/// of input messages.
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    content: String,
    message_type: MessageType,
}
