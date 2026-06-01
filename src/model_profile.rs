use crate::config::LlmProvider;

#[derive(Debug, Clone)]
pub struct ModelProfile {
    pub id: String,
    pub provider: LlmProvider,
    pub context_window_tokens: usize,
}

impl ModelProfile {
    pub fn for_id(id: &str, provider: LlmProvider) -> Self {
        if let Some(builtin) = builtin_for_id(id) {
            return ModelProfile {
                id: id.to_string(),
                provider,
                context_window_tokens: builtin.context_window_tokens,
            };
        }

        ModelProfile {
            id: id.to_string(),
            provider,
            context_window_tokens: default_context_window(provider),
        }
    }
}

struct BuiltinProfile {
    context_window_tokens: usize,
}

fn builtin_for_id(id: &str) -> Option<BuiltinProfile> {
    let normalized = normalize_id(id);
    BUILTIN_PROFILES
        .iter()
        .find(|(prefix, _)| normalized.starts_with(prefix))
        .map(|(_, profile)| BuiltinProfile {
            context_window_tokens: profile.context_window_tokens,
        })
}

fn normalize_id(id: &str) -> String {
    id.split('/').next_back().unwrap_or(id).to_ascii_lowercase()
}

fn default_context_window(provider: LlmProvider) -> usize {
    match provider {
        // Ollama default num_ctx is 2048 unless we override it. The budget
        // engine will set num_ctx to context_window_tokens when sending.
        LlmProvider::Ollama => 8_192,
        LlmProvider::OpenRouter => 32_768,
    }
}

const BUILTIN_PROFILES: &[(&str, BuiltinProfile)] = &[
    (
        "deepseek-coder-v2",
        BuiltinProfile {
            context_window_tokens: 16_384,
        },
    ),
    (
        "deepseek-coder",
        BuiltinProfile {
            context_window_tokens: 16_384,
        },
    ),
    (
        "qwen2.5-coder",
        BuiltinProfile {
            context_window_tokens: 32_768,
        },
    ),
    (
        "qwen2.5",
        BuiltinProfile {
            context_window_tokens: 32_768,
        },
    ),
    (
        "llama3.1",
        BuiltinProfile {
            context_window_tokens: 16_384,
        },
    ),
    (
        "llama3.2",
        BuiltinProfile {
            context_window_tokens: 16_384,
        },
    ),
    (
        "claude-3.5-sonnet",
        BuiltinProfile {
            context_window_tokens: 200_000,
        },
    ),
    (
        "claude-3-5-sonnet",
        BuiltinProfile {
            context_window_tokens: 200_000,
        },
    ),
    (
        "gpt-4o",
        BuiltinProfile {
            context_window_tokens: 128_000,
        },
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_matches_known_ollama_model() {
        let profile = ModelProfile::for_id("deepseek-coder-v2:latest", LlmProvider::Ollama);
        assert_eq!(profile.context_window_tokens, 16_384);
        assert_eq!(profile.provider, LlmProvider::Ollama);
        assert_eq!(profile.id, "deepseek-coder-v2:latest");
    }

    #[test]
    fn builtin_matches_through_openrouter_namespace() {
        let profile = ModelProfile::for_id(
            "anthropic/claude-3.5-sonnet",
            LlmProvider::OpenRouter,
        );
        assert_eq!(profile.context_window_tokens, 200_000);
    }

    #[test]
    fn unknown_model_falls_back_to_provider_default() {
        let ollama = ModelProfile::for_id("some-unknown-model:7b", LlmProvider::Ollama);
        assert_eq!(ollama.context_window_tokens, 8_192);

        let openrouter = ModelProfile::for_id("unknown/model", LlmProvider::OpenRouter);
        assert_eq!(openrouter.context_window_tokens, 32_768);
    }
}
