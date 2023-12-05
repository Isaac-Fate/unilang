use std::{
    pin::Pin, 
    task::{Context, Poll}
};
use anyhow::Result;
use futures::stream::{Stream, StreamExt};
use regex::Regex;
use lazy_static::lazy_static;
use bytes::Bytes;
use super::QianfanChatResponse;

lazy_static! {
    static ref STREAM_RESPONSE_CHUNK_RE: Regex = Regex::new(r"data: [^\n]+\n\n").unwrap();
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

pub struct QianfanChatResponseStream<S> {
    response_bytes_stream: S,
    remaining_content: Option<String>,
}

impl<S> QianfanChatResponseStream<S>
where 
    S: Stream<Item = Result<Bytes, reqwest::Error>> + Unpin
{
    pub fn new(response_bytes_stream: S) -> Self {
        Self { 
            response_bytes_stream, 
            remaining_content: None,
        }
    }
}

impl<S> Stream for QianfanChatResponseStream<S> 
where 
    S: Stream<Item = Result<Bytes, reqwest::Error>> + Unpin
{
    type Item = QianfanChatResponse;

    fn poll_next(
            mut self: Pin<&mut Self>, 
            cx: &mut Context<'_>
        ) -> Poll<Option<Self::Item>> {
        
        match futures::ready!(self.response_bytes_stream.poll_next_unpin(cx)) {
            Some(Ok(bytes)) => {
                // Convert bytes to string
                let mut content = std::str::from_utf8(&bytes).unwrap().to_string();

                // Concatenate the remaining content if there is any
                if let Some(remaining_content) = &self.remaining_content {
                    // Put the remaining content in front of the currently received content
                    content.insert_str(0, remaining_content);
                }

                // Extract the first response and remaining content
                let response_with_remaining_content = extract_first_response(&content).unwrap();

                // Get the response and remaining content
                let response = response_with_remaining_content.response;
                let remaining_content = response_with_remaining_content.remaining_content;
                
                // Collect the remaining content if there is any
                if let Some(remaining_content) = remaining_content {
                    self.remaining_content = Some(remaining_content);
                } else {
                    self.remaining_content = None;
                }

                // Return Ready if there is one response
                if let Some(response) = response {
                    return Poll::Ready(Some(response));
                } else {
                    return Poll::Pending;
                }
            },
            Some(Err(_)) => {
                return Poll::Ready(None);
            },
            None => {
                return Poll::Ready(None);
            }
        }
    }
}
