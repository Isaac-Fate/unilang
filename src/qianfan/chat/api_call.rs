use anyhow::{Result, anyhow};
use futures::Stream;
use reqwest::Client;
use super::super::get_access_token;
use super::QianfanChatModelName;
use super::{
    QianfanChatRequestBody,
    response::{
        QianfanChatResponse,
        QianfanChatResponseStream
    },
    QianfanError,
};

/// Call Qianfan chat API and return a complete chat response.
pub async fn get_complete_chat_response(
    client: &Client,
    model_name: QianfanChatModelName,
    request_body: &QianfanChatRequestBody
) -> Result<QianfanChatResponse> {
    // Convert to a map
    let mut request_body = serde_json::to_value(request_body)?.as_object()
        .unwrap()
        .to_owned();

    // Set the key "stream" to false
    request_body.insert(
        "stream".to_string(), serde_json::json!(false)
    );

    // Call API to get chat response
    let response = client
        .post(get_api_endpoint(model_name)?)
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

fn get_api_endpoint(model_name: QianfanChatModelName) -> Result<&'static str> {
    match model_name {
        QianfanChatModelName::ErnieBot4 => Ok("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/completions_pro"),
        QianfanChatModelName::ErnieBot => Ok("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/completions"), 
        QianfanChatModelName::ErnieBotTurbo => Ok("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/eb-instant"),
        _ => Err(anyhow!("{:?} is not supported", model_name)),

    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use futures::stream::StreamExt;
    use super::{
        get_access_token,
        get_complete_chat_response,
        get_streamed_chat_response,
        QianfanChatModelName,
        QianfanChatRequestBody,
        
    };
    use crate::qianfan::chat::{
        QianfanChatMessage,
        QianfanChatRole,
    };

    fn create_client() -> reqwest::Client {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap()
    }

    #[tokio::test]
    async fn test_get_complete_chat_response() -> Result<()> {
        // Create an HTTP client
        let client = create_client();

        // Call API to get chat response
        let response = get_complete_chat_response(
            &client,
            QianfanChatModelName::ErnieBotTurbo,
            &QianfanChatRequestBody::builder()
                .messages(vec![
                    QianfanChatMessage {
                        role: QianfanChatRole::User,
                        content: "What is Rust?".to_string(),
                    },
                ])
                .temperature(0.9)
                .build()
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
            &QianfanChatRequestBody::builder()
                .messages(vec![
                    QianfanChatMessage {
                        role: QianfanChatRole::User,
                        content: "What is Rust?".to_string(),
                    },
                ])
                .build()
        ).await?;

        while let Some(response) = response.next().await {
            println!("{:#?}", response);
        }

        Ok(())
    }
}