use super::ChatModelName;

#[derive(Debug)]
pub struct ChatModel {
    pub name: ChatModelName,
    pub temperature: f32,
    pub top_p: f32,
    pub presence_penalty: f32,
    pub profile: Option<String>,
}

impl ChatModel {
    pub fn new(
        name: ChatModelName,
        temperature: f32,
        top_p: f32,
        presence_penalty: f32,
        profile: Option<String>,
    ) -> Self {
        Self {
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
    name: ChatModelName,
    temperature: f32,
    top_p: f32,
    presence_penalty: f32,
    profile: Option<String>,
}

impl ChatModelBuilder {
    pub fn new() -> Self {
        Self {
            name: ChatModelName::OpenAIGPT3_5Turbo16K,
            temperature: 1.0,
            top_p: 1.0,
            presence_penalty: 1.0,
            profile: None,
        }
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
    pub fn profile(mut self, profile: Option<String>) -> Self {
        self.profile = profile;
        self
    }

    /// Build the chat model.
    pub fn build(self) -> ChatModel {
        ChatModel::new(
            self.name,
            self.temperature,
            self.top_p,
            self.presence_penalty,
            self.profile,
        )
    }
}