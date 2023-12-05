use std::default::Default;
use serde::Serialize;
use serde_with::skip_serializing_none;
use crate::chat::ChatMessage;


#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct QianfanChatRequestBody {
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    pub top_p: f32,
    pub penalty_score: f32,
    pub system: Option<String>,
    pub user_id: Option<String>,
}

impl QianfanChatRequestBody {
    pub fn messages(mut self, messages: Vec<ChatMessage>) -> Self {
        self.messages = messages;
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }
}

impl Default for QianfanChatRequestBody {
    /// The default values are referenced from
    /// - https://cloud.baidu.com/doc/WENXINWORKSHOP/s/clntwmv7t
    /// - https://cloud.baidu.com/doc/WENXINWORKSHOP/s/6lp69is2a
    /// - ...
    fn default() -> Self {
        Self { 
            messages: vec![], 
            temperature: 0.95, 
            top_p: 0.7,
            penalty_score: 1.0, 
            system: None, 
            user_id: None, 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::QianfanChatRequestBody;
    use serde_json::json;

    #[test]
    fn test_default() {
        let default = QianfanChatRequestBody::default();
        let expected = json!({
            "messages": [],
            "stream": false,
            "temperature": 0.95,
            "top_p": 0.7,
            "penalty_score": 1.0,
        });
        
        // assert_eq!(serde_json::to_value(default).unwrap(), expected);
        // println!("{:#?}", json!({default}));
        println!("{:#?}", serde_json::to_value(default).unwrap());
        println!("{:#?}", expected);
    }

    #[test]
    fn remove_key_value_from_json() {
        let json_object = json!({
            "name": "Isaac",
            "age": 24,
        });

        let mut map = json_object.as_object()
            .unwrap()
            .to_owned();

        map.remove("age");

        let expected = json!({
            "name": "Isaac",
        });

        assert_eq!(serde_json::to_value(map).unwrap(), expected);
    }
}