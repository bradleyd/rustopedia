use anyhow::{Context, Result, anyhow, bail};
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
        if self.config.debug {
            eprintln!(
                "[debug] llm provider={} model={} prompt_chars={}",
                self.config.llm_provider.as_str(),
                model,
                prompt.chars().count()
            );
        }
        match self.config.llm_provider {
            LlmProvider::Ollama => self.generate_ollama(model, prompt).await,
            LlmProvider::OpenRouter => self.generate_openrouter(model, prompt).await,
        }
    }

    async fn generate_ollama(&self, model: &str, prompt: &str) -> Result<String> {
        let url = format!(
            "{}/api/generate",
            self.config.ollama_base_url.trim_end_matches('/')
        );
        let profile = crate::model_profile::ModelProfile::for_id(model, LlmProvider::Ollama);
        let request = OllamaGenerateRequest {
            model,
            prompt,
            stream: false,
            options: OllamaOptions {
                num_ctx: profile.context_window_tokens,
                num_predict: self.config.llm_max_tokens as i32,
            },
        };

        let response = self
            .http_client
            .post(url)
            .json(&request)
            .send()
            .await
            .context("failed to call Ollama API")?;

        let response = ensure_success_response(response, "Ollama").await?;

        let body: OllamaGenerateResponse = response
            .json()
            .await
            .context("failed to parse Ollama response JSON")?;

        Ok(body.response.trim().to_string())
    }

    async fn generate_openrouter(&self, model: &str, prompt: &str) -> Result<String> {
        let api_key = self.config.openrouter_api_key.as_deref().ok_or_else(|| {
            anyhow!("RUSTOPEDIA_OPENROUTER_API_KEY is required for openrouter provider")
        })?;

        let url = format!(
            "{}/chat/completions",
            self.config.openrouter_base_url.trim_end_matches('/')
        );

        let request = OpenRouterChatRequest {
            model,
            messages: vec![OpenRouterMessage {
                role: "user",
                content: prompt,
            }],
            max_tokens: self.config.llm_max_tokens,
        };

        let response = self
            .http_client
            .post(url)
            .bearer_auth(api_key)
            .json(&request)
            .send()
            .await
            .context("failed to call OpenRouter API")?;

        let response = ensure_success_response(response, "OpenRouter").await?;

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

async fn ensure_success_response(
    response: reqwest::Response,
    service_name: &str,
) -> Result<reqwest::Response> {
    let status = response.status();
    if status.is_success() {
        return Ok(response);
    }

    let body = response
        .text()
        .await
        .unwrap_or_else(|_| "<failed to read response body>".to_string());

    bail!(
        "{service_name} API returned status {} with body: {}",
        status,
        truncate_for_log(&body, 800)
    );
}

fn truncate_for_log(text: &str, max_chars: usize) -> String {
    let truncated = text.chars().take(max_chars).collect::<String>();
    if text.chars().count() > max_chars {
        format!("{truncated}...")
    } else {
        truncated
    }
}

#[derive(Serialize)]
struct OllamaGenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    num_ctx: usize,
    num_predict: i32,
}

#[derive(Deserialize)]
struct OllamaGenerateResponse {
    response: String,
}

#[derive(Serialize)]
struct OpenRouterChatRequest<'a> {
    model: &'a str,
    messages: Vec<OpenRouterMessage<'a>>,
    max_tokens: u32,
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
