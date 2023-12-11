use std::time::Duration;
use reqwest::Client;
use super::ChatModelName;

#[derive(Debug)]
pub struct ChatModel {
    pub client: Client,
    pub name: ChatModelName,
    pub temperature: f32,
    pub top_p: f32,
    pub presence_penalty: f32,
    pub profile: Option<String>,
}

impl ChatModel {
    pub fn new(
        client: Client,
        name: ChatModelName,
        temperature: f32,
        top_p: f32,
        presence_penalty: f32,
        profile: Option<String>,
    ) -> Self {
        Self {
            client,
            name,
            temperature,
            top_p,
            presence_penalty,
            profile,
        }
    }

    /// Create a builder for the chat model.
    pub fn builder() -> ChatModelBuilder {
        ChatModelBuilder::new()
    }
}

pub struct ChatModelBuilder {
    client: Client,
    name: ChatModelName,
    temperature: f32,
    top_p: f32,
    presence_penalty: f32,
    profile: Option<String>,
}

impl ChatModelBuilder {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(60))
                .build()
                .unwrap(),
            name: ChatModelName::OpenAIGPT3_5Turbo16K,
            temperature: 1.0,
            top_p: 1.0,
            presence_penalty: 1.0,
            profile: None,
        }
    }

    pub fn client(mut self, client: Client) -> Self {
        self.client = client;
        self
    }

    /// Set the name of the chat model.
    pub fn name(mut self, name: ChatModelName) -> Self {
        self.name = name;
        self
    }

    /// Set the temperature of the chat model.
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = top_p;
        self
    }

    /// Set the presence penalty of the chat model.
    pub fn presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = presence_penalty;
        self
    }

    /// Set the system prompt of the chat model, which serves as the profile.
    pub fn profile<S: AsRef<str>>(mut self, profile: S) -> Self {
        self.profile = Some(profile.as_ref().to_string());
        self
    }

    /// Build the chat model.
    pub fn build(self) -> ChatModel {
        ChatModel::new(
            self.client,
            self.name,
            self.temperature,
            self.top_p,
            self.presence_penalty,
            self.profile,
        )
    }
}