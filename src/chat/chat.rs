use anyhow::{Result, anyhow};
use super::{
    ChatModel, 
    ChatModelName, 
    ChatMessage,
    ChatResponse,
};
use super::openai;

impl ChatModel {
    pub async fn get_complete_chat_response(&self, messages: Vec<ChatMessage>) -> Result<ChatResponse> {
        match self.name {
            ChatModelName::OpenAIGPT3_5Turbo 
            | ChatModelName::OpenAIGPT3_5Turbo16K 
            | ChatModelName::OpenAIGPT4 => {
                openai::get_complete_chat_response(self, messages).await
            },
            _ => {
                Err(anyhow!("{:?} is not supported", self.name))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use crate::chat::{
        ChatModel,
        ChatModelName,
        ChatMessage,
        ChatRole,
    };

    #[tokio::test]
    async fn test_get_complete_chat_response() -> Result<()> {
        // Initialize logger
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

        let model = ChatModel::builder()
            .name(ChatModelName::OpenAIGPT4)
            .build();

        let messages = vec![
            ChatMessage {
                role: ChatRole::User,
                content: "What is Rust?".to_string(),
            },
        ];

        let response = model.get_complete_chat_response(messages).await?;

        println!("{:#?}", response);

        Ok(())
    }
}