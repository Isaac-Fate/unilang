use anyhow::{Result, anyhow};
use crate::{
    chat::{
        ChatModel, 
        ChatModelName, 
        ChatMessage,
        ChatRole,
        ChatResponse,
        ChatTokenUsage,
    },
    openai::{
        self,
        chat::{
            OpenAIChatRequestBody, 
            OpenAIChatCompletion, 
            OpenAIChatMessage,
            OpenAIChatRole,
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
                .messages(
                    // Add profile to the first message if it exists
                    match &model.profile {
                        Some(profile) => vec![
                            OpenAIChatMessage {
                                role: OpenAIChatRole::System,
                                content: profile.to_string(),
                            }
                        ],
                        None => vec![],
                    }.into_iter()

                    // Messages of the user and the assistant
                    .chain(
                        messages
                        .iter()
                        .map(|message| OpenAIChatMessage {
                            role: match message.role {
                                ChatRole::User => OpenAIChatRole::User,
                                ChatRole::Assistant => OpenAIChatRole::Assistant,
                            },
                            content: message.content.to_owned(),
                        })
                    )
                    .collect::<Vec<OpenAIChatMessage>>()
                )
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
        ChatModelName::OpenAIGPT4 => Ok("gpt-4".to_string()),
        _ => Err(anyhow!("{:?} is not available in OpenAI's chat models", model_name)),
    }
}

impl From<OpenAIChatCompletion> for ChatResponse {
    fn from(response: OpenAIChatCompletion) -> Self {
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
