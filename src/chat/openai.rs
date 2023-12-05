use anyhow::{Result, anyhow};
use crate::{
    chat::{
        ChatModel, 
        ChatModelName, 
        ChatMessage,
        ChatResponse,
        ChatTokenUsage,
    },
    openai::{
        self,
        chat::{
            OpenAIChatRequestBody, OpenAIChatResponse,
        }
    },
};

pub async fn get_complete_chat_response(
    model: &ChatModel,
    messages: Vec<ChatMessage>,
) -> Result<ChatResponse> {
    // Get the model name
    let model_name = chat_model_name_to_string(&model.name)?;

    // Call API to get chat response
    Ok(
        openai::chat::get_complete_chat_response(
            &OpenAIChatRequestBody::builder()
                .model(model_name.as_str())
                .messages(messages)
                .temperature(0.9)
                .build()
        ).await?
        .into()
    )
}

/// Convert a `ChatModelName` to a `String` that can be used in OpenAI's API.
fn chat_model_name_to_string(model_name: &ChatModelName) -> Result<String> {
    match model_name {
        ChatModelName::OpenAIGPT3_5Turbo => Ok("gpt-3.5-turbo".to_string()),
        ChatModelName::OpenAIGPT3_5Turbo16K => Ok("gpt-3.5-turbo-16k".to_string()),
        _ => Err(anyhow!("{:?} is not available in OpenAI's chat models", model_name)),
    }
}

impl From<OpenAIChatResponse> for ChatResponse {
    fn from(response: OpenAIChatResponse) -> Self {
        Self {
            content: response.choices
                .first()
                .unwrap()
                .message
                .content
                .to_owned(),
            is_complete: true,
            usage: ChatTokenUsage { 
                prompt_tokens: response.usage.prompt_tokens, 
                completion_tokens: response.usage.completion_tokens, 
                total_tokens: response.usage.total_tokens,
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
    use super::get_complete_chat_response;

    #[tokio::test]
    async fn test_get_complete_chat_response() -> Result<()> {
        let response = get_complete_chat_response(
            &ChatModel::builder()
                .name(ChatModelName::OpenAIGPT3_5Turbo16K)
                .temperature(0.1)
                .build(),
            vec![
                ChatMessage {
                    role: ChatRole::User,
                    content: "What is Rust?".to_string(),
                },
            ],
        ).await?;

        println!("{:#?}", response);

        Ok(())
    }
}
