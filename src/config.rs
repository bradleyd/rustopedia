use std::env;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmProvider {
    Ollama,
    OpenRouter,
}

impl LlmProvider {
    fn from_env() -> Self {
        match env::var("RUSTOPEDIA_LLM_PROVIDER")
            .unwrap_or_else(|_| "ollama".to_string())
            .to_lowercase()
            .as_str()
        {
            "openrouter" => Self::OpenRouter,
            _ => Self::Ollama,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ollama => "ollama",
            Self::OpenRouter => "openrouter",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub model_name: String,
    pub review_model_name: String,
    pub edit_model_name: String,
    pub llm_provider: LlmProvider,
    pub ollama_base_url: String,
    pub openrouter_base_url: String,
    pub openrouter_api_key: Option<String>,
    pub qdrant_url: String,
    pub rag_top_k: usize,
    pub http_connect_timeout_secs: u64,
    pub http_request_timeout_secs: u64,
    pub embed_query_timeout_secs: u64,
    pub rust_analyzer_bin: String,
    pub rust_analyzer_timeout_secs: u64,
    pub cargo_timeout_secs: u64,
    pub ripgrep_bin: String,
    pub edit_max_retries: u32,
    pub llm_max_tokens: u32,
    /// Hard ceiling on the assembled prompt size (tokens). When > 0, lowest-priority
    /// context is dropped until the prompt fits — useful for local servers with a
    /// prefill memory cap. 0 disables the guard.
    pub max_prompt_tokens: usize,
    pub python_bin: String,
    pub embed_query_script: String,
    pub project_root: String,
    pub debug: bool,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let model_name = env::var("RUSTOPEDIA_MODEL_NAME")
            .unwrap_or_else(|_| "deepseek-coder-v2:latest".to_string());
        Self {
            review_model_name: env::var("RUSTOPEDIA_REVIEW_MODEL_NAME")
                .unwrap_or_else(|_| model_name.clone()),
            edit_model_name: env::var("RUSTOPEDIA_EDIT_MODEL_NAME")
                .unwrap_or_else(|_| model_name.clone()),
            model_name,
            llm_provider: LlmProvider::from_env(),
            ollama_base_url: env::var("RUSTOPEDIA_OLLAMA_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:11434".to_string()),
            openrouter_base_url: env::var("RUSTOPEDIA_OPENROUTER_BASE_URL")
                .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string()),
            openrouter_api_key: env::var("RUSTOPEDIA_OPENROUTER_API_KEY").ok(),
            qdrant_url: env::var("RUSTOPEDIA_QDRANT_URL")
                .unwrap_or_else(|_| "http://localhost:6333".to_string()),
            rag_top_k: env::var("RUSTOPEDIA_RAG_TOP_K")
                .ok()
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(10),
            http_connect_timeout_secs: env::var("RUSTOPEDIA_HTTP_CONNECT_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(10),
            http_request_timeout_secs: env::var("RUSTOPEDIA_HTTP_REQUEST_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(30),
            embed_query_timeout_secs: env::var("RUSTOPEDIA_EMBED_QUERY_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(30),
            rust_analyzer_bin: env::var("RUSTOPEDIA_RUST_ANALYZER_BIN")
                .unwrap_or_else(|_| "rust-analyzer".to_string()),
            rust_analyzer_timeout_secs: env::var("RUSTOPEDIA_RUST_ANALYZER_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(20),
            cargo_timeout_secs: env::var("RUSTOPEDIA_CARGO_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(120),
            ripgrep_bin: env::var("RUSTOPEDIA_RIPGREP_BIN").unwrap_or_else(|_| "rg".to_string()),
            edit_max_retries: env::var("RUSTOPEDIA_EDIT_MAX_RETRIES")
                .ok()
                .and_then(|v| v.parse::<u32>().ok())
                .unwrap_or(2),
            llm_max_tokens: env::var("RUSTOPEDIA_LLM_MAX_TOKENS")
                .ok()
                .and_then(|v| v.parse::<u32>().ok())
                .unwrap_or(4096),
            max_prompt_tokens: env::var("RUSTOPEDIA_MAX_PROMPT_TOKENS")
                .ok()
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(0),
            python_bin: env::var("RUSTOPEDIA_PYTHON_BIN").unwrap_or_else(|_| "python3".to_string()),
            embed_query_script: env::var("RUSTOPEDIA_EMBED_QUERY_SCRIPT")
                .unwrap_or_else(|_| "rag/embed_query.py".to_string()),
            project_root: env::var("RUSTOPEDIA_PROJECT_ROOT").unwrap_or_else(|_| ".".to_string()),
            debug: env::var("RUSTOPEDIA_DEBUG").is_ok(),
        }
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.model_name.trim().is_empty() {
            anyhow::bail!("RUSTOPEDIA_MODEL_NAME cannot be empty");
        }
        if self.review_model_name.trim().is_empty() {
            anyhow::bail!("RUSTOPEDIA_REVIEW_MODEL_NAME cannot be empty");
        }
        if self.edit_model_name.trim().is_empty() {
            anyhow::bail!("RUSTOPEDIA_EDIT_MODEL_NAME cannot be empty");
        }
        if self.qdrant_url.trim().is_empty() {
            anyhow::bail!("RUSTOPEDIA_QDRANT_URL cannot be empty");
        }
        if self.rag_top_k == 0 {
            anyhow::bail!("RUSTOPEDIA_RAG_TOP_K must be > 0");
        }
        if self.http_connect_timeout_secs == 0 {
            anyhow::bail!("RUSTOPEDIA_HTTP_CONNECT_TIMEOUT_SECS must be > 0");
        }
        if self.http_request_timeout_secs == 0 {
            anyhow::bail!("RUSTOPEDIA_HTTP_REQUEST_TIMEOUT_SECS must be > 0");
        }
        if self.embed_query_timeout_secs == 0 {
            anyhow::bail!("RUSTOPEDIA_EMBED_QUERY_TIMEOUT_SECS must be > 0");
        }
        if self.rust_analyzer_bin.trim().is_empty() {
            anyhow::bail!("RUSTOPEDIA_RUST_ANALYZER_BIN cannot be empty");
        }
        if self.rust_analyzer_timeout_secs == 0 {
            anyhow::bail!("RUSTOPEDIA_RUST_ANALYZER_TIMEOUT_SECS must be > 0");
        }
        if self.cargo_timeout_secs == 0 {
            anyhow::bail!("RUSTOPEDIA_CARGO_TIMEOUT_SECS must be > 0");
        }
        if self.ripgrep_bin.trim().is_empty() {
            anyhow::bail!("RUSTOPEDIA_RIPGREP_BIN cannot be empty");
        }
        if self.llm_max_tokens == 0 {
            anyhow::bail!("RUSTOPEDIA_LLM_MAX_TOKENS must be > 0");
        }
        if self.python_bin.trim().is_empty() {
            anyhow::bail!("RUSTOPEDIA_PYTHON_BIN cannot be empty");
        }
        if self.embed_query_script.trim().is_empty() {
            anyhow::bail!("RUSTOPEDIA_EMBED_QUERY_SCRIPT cannot be empty");
        }

        match self.llm_provider {
            LlmProvider::Ollama => {
                if self.ollama_base_url.trim().is_empty() {
                    anyhow::bail!("RUSTOPEDIA_OLLAMA_BASE_URL cannot be empty");
                }
            }
            LlmProvider::OpenRouter => {
                if self.openrouter_base_url.trim().is_empty() {
                    anyhow::bail!("RUSTOPEDIA_OPENROUTER_BASE_URL cannot be empty");
                }
                if self
                    .openrouter_api_key
                    .as_deref()
                    .is_none_or(|key| key.trim().is_empty())
                {
                    anyhow::bail!(
                        "RUSTOPEDIA_OPENROUTER_API_KEY is required when RUSTOPEDIA_LLM_PROVIDER=openrouter"
                    );
                }
            }
        }

        Ok(())
    }

    pub fn warn_suspicious_config(&self) {
        const OPENROUTER_CLOUD_HOST: &str = "openrouter.ai";

        if matches!(self.llm_provider, LlmProvider::OpenRouter)
            && self.openrouter_base_url.contains(OPENROUTER_CLOUD_HOST)
        {
            let key_looks_like_placeholder = self
                .openrouter_api_key
                .as_deref()
                .is_some_and(|key| key.len() < 20 || !key.starts_with("sk-"));

            if key_looks_like_placeholder {
                eprintln!(
                    "⚠️ openrouter provider selected with cloud default URL ({}) but RUSTOPEDIA_OPENROUTER_API_KEY does not look like a real OpenRouter key.",
                    self.openrouter_base_url
                );
                eprintln!(
                    "   If you meant to use a local OpenAI-compatible server (mlx, vllm, etc.), set RUSTOPEDIA_OPENROUTER_BASE_URL to its endpoint (e.g. http://127.0.0.1:8000/v1)."
                );
            }
        }
    }

    pub fn http_connect_timeout(&self) -> Duration {
        Duration::from_secs(self.http_connect_timeout_secs)
    }

    pub fn http_request_timeout(&self) -> Duration {
        Duration::from_secs(self.http_request_timeout_secs)
    }

    pub fn embed_query_timeout(&self) -> Duration {
        Duration::from_secs(self.embed_query_timeout_secs)
    }

    pub fn rust_analyzer_timeout(&self) -> Duration {
        Duration::from_secs(self.rust_analyzer_timeout_secs)
    }

    pub fn cargo_timeout(&self) -> Duration {
        Duration::from_secs(self.cargo_timeout_secs)
    }

    pub fn model_for_mode(&self, mode: crate::session::SessionMode) -> &str {
        match mode {
            crate::session::SessionMode::Ask => &self.model_name,
            crate::session::SessionMode::Review => &self.review_model_name,
            crate::session::SessionMode::Edit => &self.edit_model_name,
        }
    }

    pub fn profile_for_mode(
        &self,
        mode: crate::session::SessionMode,
    ) -> crate::model_profile::ModelProfile {
        crate::model_profile::ModelProfile::for_id(self.model_for_mode(mode), self.llm_provider)
    }

    pub fn build_http_client(&self) -> anyhow::Result<reqwest::Client> {
        reqwest::Client::builder()
            .connect_timeout(self.http_connect_timeout())
            .timeout(self.http_request_timeout())
            .build()
            .map_err(Into::into)
    }
}
