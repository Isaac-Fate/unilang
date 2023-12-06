use serde::Serialize;
use serde_with::skip_serializing_none;
use super::QianfanChatMessage;

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct QianfanChatRequestBody {
    pub messages: Vec<QianfanChatMessage>,
    pub temperature: f32,
    pub top_p: f32,
    pub penalty_score: f32,
    pub system: Option<String>,
    pub user_id: Option<String>,
}

impl QianfanChatRequestBody {
    pub fn builder() -> QianfanChatRequestBodyBuilder {
        QianfanChatRequestBodyBuilder::new()
    }
}

pub struct QianfanChatRequestBodyBuilder {
    messages: Vec<QianfanChatMessage>,
    temperature: f32,
    top_p: f32,
    penalty_score: f32,
    system: Option<String>,
    user_id: Option<String>,
}

impl QianfanChatRequestBodyBuilder {
    /// Create a new builder with default values.
    pub fn new() -> Self {
        Self {
            messages: vec![],
            temperature: 0.95,
            top_p: 0.7,
            penalty_score: 1.0,
            system: None,
            user_id: None,
        }
    }

    pub fn messages(mut self, messages: Vec<QianfanChatMessage>) -> Self {
        self.messages = messages;
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = top_p;
        self
    }

    pub fn penalty_score(mut self, penalty_score: f32) -> Self {
        self.penalty_score = penalty_score;
        self
    }

    pub fn system<S: AsRef<str>>(mut self, system: S) -> Self {
        self.system = Some(system.as_ref().to_string());
        self
    }

    pub fn user_id<S: AsRef<str>>(mut self, user_id: S) -> Self {
        self.user_id = Some(user_id.as_ref().to_string());
        self
    }

    pub fn build(self) -> QianfanChatRequestBody {
        QianfanChatRequestBody {
            messages: self.messages,
            temperature: self.temperature,
            top_p: self.top_p,
            penalty_score: self.penalty_score,
            system: self.system,
            user_id: self.user_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::QianfanChatRequestBody;
    use serde_json::json;

    #[test]
    fn test_builder() {
        let default = QianfanChatRequestBody::builder().build();
        let expected = json!({
            "messages": [],
            "stream": false,
            "temperature": 0.95,
            "top_p": 0.7,
            "penalty_score": 1.0,
        });
        
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