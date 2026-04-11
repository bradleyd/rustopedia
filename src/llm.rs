use anyhow::{Context, Result, anyhow, bail};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::config::{AppConfig, LlmProvider};

#[derive(Clone)]
pub struct LlmClient {
    http_client: reqwest::Client,
    config: AppConfig,
}

impl LlmClient {
    pub fn new(config: AppConfig) -> Result<Self> {
        Ok(Self {
            http_client: config
                .build_http_client()
                .context("failed to build LLM HTTP client")?,
            config,
        })
    }

    pub async fn generate(&self, model: &str, prompt: &str) -> Result<String> {
        match self.config.llm_provider {
            LlmProvider::Ollama => self.generate_ollama(model, prompt).await,
            LlmProvider::OpenRouter => self.generate_openrouter(model, prompt).await,
        }
    }

    async fn generate_ollama(&self, model: &str, prompt: &str) -> Result<String> {
        let url = format!("{}/api/generate", self.config.ollama_base_url.trim_end_matches('/'));
        let request = OllamaGenerateRequest {
            model,
            prompt,
            stream: false,
        };

        let response = self
            .http_client
            .post(url)
            .json(&request)
            .send()
            .await
            .context("failed to call Ollama API")?
            .error_for_status()
            .context("Ollama API returned non-success status")?;

        let body: OllamaGenerateResponse = response
            .json()
            .await
            .context("failed to parse Ollama response JSON")?;

        Ok(body.response.trim().to_string())
    }

    async fn generate_openrouter(&self, model: &str, prompt: &str) -> Result<String> {
        let api_key = self
            .config
            .openrouter_api_key
            .as_deref()
            .ok_or_else(|| anyhow!("RUSTOPEDIA_OPENROUTER_API_KEY is required for openrouter provider"))?;

        let url = format!(
            "{}/chat/completions",
            self.config.openrouter_base_url.trim_end_matches('/')
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {api_key}"))
                .context("failed to build OpenRouter authorization header")?,
        );

        let request = OpenRouterChatRequest {
            model,
            messages: vec![OpenRouterMessage {
                role: "user",
                content: prompt,
            }],
        };

        let response = self
            .http_client
            .post(url)
            .headers(headers)
            .json(&request)
            .send()
            .await
            .context("failed to call OpenRouter API")?
            .error_for_status()
            .context("OpenRouter API returned non-success status")?;

        let body: OpenRouterChatResponse = response
            .json()
            .await
            .context("failed to parse OpenRouter response JSON")?;

        let content = body
            .choices
            .first()
            .map(|choice| choice.message.content.trim())
            .unwrap_or_default();

        if content.is_empty() {
            bail!("OpenRouter response contained no message content");
        }

        Ok(content.to_string())
    }
}

#[derive(Serialize)]
struct OllamaGenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaGenerateResponse {
    response: String,
}

#[derive(Serialize)]
struct OpenRouterChatRequest<'a> {
    model: &'a str,
    messages: Vec<OpenRouterMessage<'a>>,
}

#[derive(Serialize)]
struct OpenRouterMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct OpenRouterChatResponse {
    choices: Vec<OpenRouterChoice>,
}

#[derive(Deserialize)]
struct OpenRouterChoice {
    message: OpenRouterAssistantMessage,
}

#[derive(Deserialize)]
struct OpenRouterAssistantMessage {
    content: String,
}
