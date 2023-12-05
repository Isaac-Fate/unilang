use serde::Serialize;
use super::ChatRole;

#[derive(Debug, Serialize)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::ChatMessage;
    use super::ChatRole;

    #[test]
    fn test_serialize_chat_message() {
        let default = ChatMessage {
            role: ChatRole::User,
            content: "Hello, world!".to_string(),
        };
        let expected = r#"{"role":"user","content":"Hello, world!"}"#;

        assert_eq!(serde_json::to_string(&default).unwrap(), expected);
    }
}