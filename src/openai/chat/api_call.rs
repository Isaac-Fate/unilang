use anyhow::{Result, anyhow};
use super::{
    super::OPENAI_API_KEY,
    OpenAIChatRequestBody,
    OpenAIChatResponse,
};

/// Call OpenAI chat API and return a complete chat response.
pub async fn get_complete_chat_response(request_body: &OpenAIChatRequestBody) -> Result<OpenAIChatResponse> {
    // Convert to a map
    let mut request_body = serde_json::to_value(request_body)?.as_object()
        .unwrap()
        .to_owned();

    // Set the key "stream" to false
    request_body.insert(
        "stream".to_string(), serde_json::json!(false)
    );

    // Call API to get chat response
    let response = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", OPENAI_API_KEY.as_str()))
        .json(&request_body)
        .send()
        .await?;

    // Get the response content
    let response_content = response.text().await?;

    // Parse the response content
    // If the response is successful, parse the response content as OpenAIChatResponse
    // If the response is not successful, parse the response content as QianfanError
    if let Ok(response) = serde_json::from_str::<OpenAIChatResponse>(&response_content) {
        Ok(response)
    } else {
        // Err(
        //     serde_json::from_str::<QianfanError>(&response_content)?.into()
        // )
        Err(anyhow!(response_content))
    }
}

// /// Call Qianfan chat API and return a stream of chat responses.
// pub async fn get_streamed_chat_response(request_body: &OpenAIChatRequestBody) -> Result<impl Stream<Item = OpenAIChatResponse>> {
//     todo!()
// }


#[cfg(test)]
mod tests {
    use anyhow::Result;
    use crate::chat::{
        ChatMessage,
        ChatRole,
    };
    use super::{
        get_complete_chat_response,
        // get_streamed_chat_response,
        OpenAIChatRequestBody,
    };

    #[tokio::test]
    async fn test_get_complete_chat_response() -> Result<()> {


        // Call API to get chat response
        let response = get_complete_chat_response(
            &OpenAIChatRequestBody::builder()
                .messages(vec![
                    ChatMessage {
                        role: ChatRole::User,
                        content: "What is Rust?".to_string(),
                    },
                ])
                .temperature(0.9)
                .build()
        ).await;

        println!("{:#?}", response);

        Ok(())
    }

    // #[tokio::test]
    // async fn test_get_streamed_chat_response() -> Result<()> {

    //     // Call API to get chat response
    //     let mut response = get_streamed_chat_response(
    //         &OpenAIChatRequestBody::default()
    //             .messages(vec![
    //                 ChatMessage {
    //                     role: ChatRole::User,
    //                     content: "What is Rust?".to_string(),
    //                 },
    //             ])
    //     ).await?;

    //     while let Some(response) = response.next().await {
    //         println!("{:#?}", response);
    //     }

    //     Ok(())
    // }
}