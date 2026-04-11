use std::env;

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
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub model_name: String,
    pub planner_model_name: String,
    pub llm_provider: LlmProvider,
    pub ollama_base_url: String,
    pub openrouter_base_url: String,
    pub openrouter_api_key: Option<String>,
    pub qdrant_url: String,
    pub rag_top_k: usize,
    pub python_bin: String,
    pub embed_query_script: String,
    pub project_root: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let model_name =
            env::var("RUSTOPEDIA_MODEL_NAME").unwrap_or_else(|_| "openhermes".to_string());
        Self {
            planner_model_name: env::var("RUSTOPEDIA_PLANNER_MODEL_NAME")
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
            python_bin: env::var("RUSTOPEDIA_PYTHON_BIN").unwrap_or_else(|_| "python3".to_string()),
            embed_query_script: env::var("RUSTOPEDIA_EMBED_QUERY_SCRIPT")
                .unwrap_or_else(|_| "rag/embed_query.py".to_string()),
            project_root: env::var("RUSTOPEDIA_PROJECT_ROOT").unwrap_or_else(|_| ".".to_string()),
        }
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.model_name.trim().is_empty() {
            anyhow::bail!("RUSTOPEDIA_MODEL_NAME cannot be empty");
        }
        if self.planner_model_name.trim().is_empty() {
            anyhow::bail!("RUSTOPEDIA_PLANNER_MODEL_NAME cannot be empty");
        }
        if self.qdrant_url.trim().is_empty() {
            anyhow::bail!("RUSTOPEDIA_QDRANT_URL cannot be empty");
        }
        if self.rag_top_k == 0 {
            anyhow::bail!("RUSTOPEDIA_RAG_TOP_K must be > 0");
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
}
