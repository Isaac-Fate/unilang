use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ChatRole {
    #[serde(rename = "user")]
    User,

    #[serde(rename = "assistant")]
    Assistant,
}
