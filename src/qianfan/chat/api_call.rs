use anyhow::Result;
use futures::stream::Stream;
use super::super::get_access_token;
use super::{
    QianfanChatRequestBody,
    response::{
        QianfanChatResponse,
        QianfanChatResponseStream
    },
    QianfanError,
};

/// Call Qianfan chat API and return a complete chat response.
pub async fn get_complete_chat_response(request_body: &QianfanChatRequestBody) -> Result<QianfanChatResponse> {
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
        .post("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/eb-instant")
        .query(&[
            ("access_token", get_access_token().await?.as_str()),
        ])
        .json(&request_body)
        .send()
        .await?;

    // Get the response content
    let response_content = response.text().await?;

    // Parse the response content
    // If the response is successful, parse the response content as QianfanChatResponse
    // If the response is not successful, parse the response content as QianfanError
    if let Ok(response) = serde_json::from_str::<QianfanChatResponse>(&response_content) {
        Ok(response)
    } else {
        Err(
            serde_json::from_str::<QianfanError>(&response_content)?.into()
        )
    }
}

/// Call Qianfan chat API and return a stream of chat responses.
pub async fn get_streamed_chat_response(request_body: &QianfanChatRequestBody) -> Result<impl Stream<Item = QianfanChatResponse>> {
    // Convert to a map
    let mut request_body = serde_json::to_value(request_body)?.as_object()
        .unwrap()
        .to_owned();

    // Set the key "stream" to false
    request_body.insert(
        "stream".to_string(), serde_json::json!(true)
    );

    // Call API to get chat response
    let response = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?
        .post("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/eb-instant")
        .query(&[
            ("access_token", get_access_token().await?.as_str()),
        ])
        .json(&request_body)
        .send()
        .await?;

    // Create ChatResponseStream from the response bytes stream
    Ok(
        QianfanChatResponseStream::new(response.bytes_stream())
    )
}


#[cfg(test)]
mod tests {
    use anyhow::Result;
    use futures::stream::StreamExt;
    use crate::chat::{
        ChatMessage,
        ChatRole,
    };
    use super::{
        get_access_token,
        get_complete_chat_response,
        get_streamed_chat_response,
        QianfanChatRequestBody,
    };

    #[tokio::test]
    async fn test_get_complete_chat_response() -> Result<()> {

        let access_token = get_access_token().await?;
        println!("access_token: {}", access_token);

        // Call API to get chat response
        let response = get_complete_chat_response(
            &QianfanChatRequestBody::default()
                .messages(vec![
                    ChatMessage {
                        role: ChatRole::User,
                        content: "What is Rust?".to_string(),
                    },
                ])
                .temperature(0.9)
        ).await;

        println!("{:#?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_streamed_chat_response() -> Result<()> {

        let access_token = get_access_token().await?;
        println!("access_token: {}", access_token);

        // Call API to get chat response
        let mut response = get_streamed_chat_response(
            &QianfanChatRequestBody::default()
                .messages(vec![
                    ChatMessage {
                        role: ChatRole::User,
                        content: "What is Rust?".to_string(),
                    },
                ])
        ).await?;

        while let Some(response) = response.next().await {
            println!("{:#?}", response);
        }

        Ok(())
    }
}