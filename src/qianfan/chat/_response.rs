use anyhow::Result;
use regex::Regex;
use lazy_static::lazy_static;
use bytes::Bytes;
use serde::Deserialize;

lazy_static! {
    static ref STREAM_RESPONSE_CHUNK_RE: Regex = Regex::new(r"data: [^\n]+\n\n").unwrap();
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct QianfanChatResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub sentence_id: Option<u32>,
    pub is_end: Option<bool>,
    pub is_truncated: bool,
    pub result: String,
    pub need_clear_history: bool,
    pub usage: Usage,
}

impl TryFrom<Bytes> for QianfanChatResponse {
    type Error = QianfanChatResponseParseError;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        if let Some(bytes) = &bytes.strip_prefix(b"data:") {
            match serde_json::from_slice::<QianfanChatResponse>(bytes) {
                Ok(response) => Ok(response),
                Err(error) => Err(QianfanChatResponseParseError::JsonParseError(error)),
            }
        } else {
            match serde_json::from_slice::<QianfanChatResponse>(&bytes) {
                Ok(response) => Ok(response),
                Err(_) => Err(QianfanChatResponseParseError::MissingDataPrefix),
            }
        }
    }
}

pub fn extract_first_response(content: &str) -> Result<ExtractedResponseWithRemainingContent> {
    if let Some(mat) = STREAM_RESPONSE_CHUNK_RE.find(content) {
        // Matched string
        let matched_str = mat.as_str();

        // Remaining content
        let remaining_content = &content[mat.end()..];

        // Extract the JSON content
        let json_content = matched_str.strip_prefix("data: ").unwrap();

        // Parse the JSON content to QianfanChatResponse
        let response = serde_json::from_str(json_content)?;

        // Return the extracted response and remaining content
        Ok(
            ExtractedResponseWithRemainingContent {
                response: Some(response),

                // If there is no remaining content, return None
                remaining_content: match remaining_content.len() {
                    0 => None,
                    _ => Some(remaining_content.to_string()),
                },
            }
        )
    } else {
        // There is no response in the json_content,
        // so return None for the response and the json_content as the remaining content
        Ok(
            ExtractedResponseWithRemainingContent {
                response: None,
                remaining_content: match content.len() {
                    0 => None,
                    _ => Some(content.to_string()),
                },
            }
        )
    }
}

pub struct ExtractedResponseWithRemainingContent {
    pub response: Option<QianfanChatResponse>,
    pub remaining_content: Option<String>,
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to parse bytes to QianfanChatResponse")]
pub enum QianfanChatResponseParseError {
    #[error("Missing 'data: ' prefix")]
    MissingDataPrefix,

    #[error("Failed to parse bytes to QianfanChatResponse")]
    JsonParseError(#[from] serde_json::Error),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use std::convert::TryFrom;
    use crate::qianfan::chat::response::STREAM_RESPONSE_CHUNK_RE;

    use super::QianfanChatResponse;

    #[test]
    fn test_stream_response_re() {
        // Dummy bytes received from the Qianfan API
        let bytes = Bytes::from_static(
            b"data: {\"id\":\"session:1\",\"object\":\"text_completion\",\"created\":1625247049,\"sentence_id\":1,\"is_end\":false,\"is_truncated\":false,\"result\":\"Rust is a multi-paradigm programming language designed for performance and safety, especially safe concurrency.\",\"need_clear_history\":false,\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":100,\"total_tokens\":110}}\n\ndata: "
        );

        // Convert to string slice
        let content = std::str::from_utf8(&bytes).unwrap();
        
        // The found Match object
        let mat = STREAM_RESPONSE_CHUNK_RE.find(content).unwrap();

        println!("{:?}", &content[mat.end()..]);

        // println!("{:?}", Bytes::from_static(json_content.as_bytes()));

        // Matched string
        let matched_str = mat.as_str();

        println!(
            "matched string: {:?}", 
            matched_str
        );
    }

    #[tokio::test]
    async fn test_parse_to_qianfan_chat_response() {

        // The expected response
        let expected_response = QianfanChatResponse {
            id: "session:1".to_string(),
            object: "text_completion".to_string(),
            created: 1625247049,
            sentence_id: Some(1),
            is_end: Some(false),
            is_truncated: false,
            result: "Rust is a multi-paradigm programming language designed for performance and safety, especially safe concurrency.".to_string(),
            need_clear_history: false,
            usage: super::Usage {
                prompt_tokens: 10,
                completion_tokens: 100,
                total_tokens: 110,
            },
        };

        // The bytes to parse
        let bytes = Bytes::from_static(
            b"data: {\"id\":\"session:1\",\"object\":\"text_completion\",\"created\":1625247049,\"sentence_id\":1,\"is_end\":false,\"is_truncated\":false,\"result\":\"Rust is a multi-paradigm programming language designed for performance and safety, especially safe concurrency.\",\"need_clear_history\":false,\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":100,\"total_tokens\":110}}",
        );

        // Parse the bytes to QianfanChatResponse
        let response = QianfanChatResponse::try_from(bytes).unwrap();

        // Check if the response is equal to the expected response
        assert_eq!(response, expected_response);

        let response = QianfanChatResponse::try_from(
            Bytes::from_static(
                b"result: {\"id\":\"session:1\",\"object\":\"text_completion\",\"created\":1625247049,\"sentence_id\":1,\"is_end\":false,\"is_truncated\":false,\"result\":\"Rust is a multi-paradigm programming language designed for performance and safety, especially safe concurrency.\",\"need_clear_history\":false,\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":100,\"total_tokens\":110}}"
            )
        );

        // This response is error because it is missing the 'data: ' prefix
        assert!(response.is_err());
        println!("response: {:?}", response);


        
    }
}
