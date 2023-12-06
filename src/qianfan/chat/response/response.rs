use serde::Deserialize;
use bytes::Bytes;
use super::QianfanChatTokenUsage;

#[derive(Debug, Deserialize)]
pub struct QianfanChatResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub sentence_id: Option<u32>,
    pub is_end: Option<bool>,
    pub is_truncated: bool,
    pub result: String,
    pub need_clear_history: bool,
    pub usage: QianfanChatTokenUsage,
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

#[derive(Debug, thiserror::Error)]
#[error("Failed to parse bytes to QianfanChatResponse")]
pub enum QianfanChatResponseParseError {
    #[error("Missing 'data: ' prefix")]
    MissingDataPrefix,

    #[error("Failed to parse bytes to QianfanChatResponse")]
    JsonParseError(#[from] serde_json::Error),
}

