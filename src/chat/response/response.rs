use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub content: String,
    pub is_complete: bool,
    pub usage: ChatTokenUsage,
}

#[derive(Debug, Deserialize)]
pub struct ChatTokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
