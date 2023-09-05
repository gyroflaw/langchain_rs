use anyhow::Result;
use async_openai::config::OpenAIConfig;
use async_openai::types::ChatCompletionRequestMessageArgs;
use async_openai::types::CreateChatCompletionRequestArgs;
use async_openai::types::Role;
use async_openai::Client;
use tiktoken_rs::r50k_base;

use crate::schema::GenerateResult;
use crate::schema::TokenUsage;
use crate::schema::LLM;
pub enum OpenAIModel {
    Gpt35,
    Gpt4,
}

impl ToString for OpenAIModel {
    fn to_string(&self) -> String {
        match self {
            OpenAIModel::Gpt35 => "gpt-3.5-turbo".to_string(),
            OpenAIModel::Gpt4 => "gpt-4".to_string(),
        }
    }
}

pub struct OpenAI {
    config: OpenAIConfig,
    model: OpenAIModel,
    max_tokens: u16,
    temperature: f32,
    top_p: f32,
    frequency_penalty: f32,
    presence_penalty: f32,
}

impl Default for OpenAI {
    fn default() -> Self {
        Self {
            config: OpenAIConfig::default(),
            model: OpenAIModel::Gpt35,
            max_tokens: 256,
            temperature: 0.7,
            top_p: 1.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
        }
    }
}

impl OpenAI {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_config(config: OpenAIConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    pub fn with_model(mut self, model: OpenAIModel) -> Self {
        self.model = model;
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u16) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = top_p;
        self
    }

    pub fn with_frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.frequency_penalty = frequency_penalty;
        self
    }

    pub fn with_presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = presence_penalty;
        self
    }

    pub fn get_client(&self) -> Client<OpenAIConfig> {
        Client::with_config(self.config.clone())
    }
}

#[async_trait::async_trait]
impl LLM for OpenAI {
    async fn generate(&self, text: &str) -> Result<GenerateResult> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(self.model.to_string())
            .max_tokens(self.max_tokens - text.len() as u16)
            .temperature(self.temperature)
            .top_p(self.top_p)
            .frequency_penalty(self.frequency_penalty)
            .presence_penalty(self.presence_penalty)
            .messages([ChatCompletionRequestMessageArgs::default()
                .role(Role::Assistant)
                .content(text)
                .build()?])
            .build()?;

        let client = self.get_client();
        let response = client.chat().create(request).await?;

        let mut generate_result = GenerateResult::default();

        if let Some(usage) = response.usage {
            generate_result.tokens = Some(TokenUsage {
                prompt_tokens: usage.prompt_tokens,
                completion_tokens: usage.completion_tokens,
                total_tokens: usage.total_tokens,
            });
        }

        if let Some(choice) = response.choices.first() {
            generate_result.generation = choice.message.content.clone().unwrap_or_default();
        } else {
            generate_result.generation = "".to_string();
        }

        Ok(generate_result)
    }

    fn tokenize(&self, text: &str) -> Result<Vec<String>> {
        // TODO: select appropriate tokenizer based on model
        let rke = r50k_base()?;
        let splitted_text = rke.split_by_token(text, true)?;
        Ok(splitted_text)
    }
}
