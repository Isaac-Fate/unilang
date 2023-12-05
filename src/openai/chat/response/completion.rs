use serde::Deserialize;
use super::OpenAIChatTokenUsage;

#[derive(Debug, Deserialize)]
pub struct OpenAIChatCompletion {
    pub id: String,
    pub choices: Vec<OpenAIChatResponseChoice>,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: OpenAIChatTokenUsage,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChatResponseChoice {
    pub finish_reason: String,
    pub index: u32,
    pub message: OpenAIChatResponseMessage,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChatResponseMessage {
    pub content: String,
    pub role: String,
}
