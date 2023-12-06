use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QianfanChatMessage {
    
    pub role: QianfanChatRole,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QianfanChatRole {
    #[serde(rename = "user")]
    User,

    #[serde(rename = "assistant")]
    Assistant,
}