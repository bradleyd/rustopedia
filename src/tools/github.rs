use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde_json::Value;

use crate::config::AppConfig;

fn classify_query(query: &str) -> &'static str {
    let q = query.to_lowercase();
    if q.contains("cli") || q.contains("command line") {
        "cli"
    } else if q.contains("json") || q.contains("serialize") {
        "serialization"
    } else if q.contains("web") || q.contains("http") {
        "web"
    } else {
        "general"
    }
}

async fn search_github_crates(
    topic: &str,
    limit: usize,
    config: &AppConfig,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let client = config.build_http_client()?;
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("rust-llm-agent"));
    let url = format!(
        "https://api.github.com/search/repositories?q={}+language:rust&sort=stars&order=desc&per_page={}",
        topic, limit
    );
    let response = client.get(&url).headers(headers).send().await?;
    let json: Value = response.json().await?;
    let repos = json["items"]
        .as_array()
        .ok_or("No repositories found")?
        .iter()
        .filter_map(|repo| {
            let full_name = repo["full_name"].as_str()?.to_string();
            let html_url = repo["html_url"].as_str()?.to_string();
            Some((full_name, html_url))
        })
        .collect();
    Ok(repos)
}

async fn fetch_readme(
    repo: &str,
    config: &AppConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = config.build_http_client()?;
    let url = format!("https://api.github.com/repos/{}/readme", repo);

    let response = client
        .get(&url)
        .header(USER_AGENT, "rust-llm-agent")
        .header("Accept", "application/vnd.github.v3.raw")
        .send()
        .await?;

    Ok(truncate_readme(&response.text().await?, 1200))
}

pub async fn search_github(
    query: &str,
    config: &AppConfig,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let topic = classify_query(query);
    match search_github_crates(topic, 3, config).await {
        Ok(repos) => {
            let mut examples = Vec::new();
            for (repo, url) in repos {
                if let Ok(readme) = fetch_readme(&repo, config).await {
                    examples.push(serde_json::json!({
                        "crate": repo,
                        "url": url,
                        "readme": readme
                    }));
                }
            }
            Ok(serde_json::json!({ "examples": examples }))
        }
        Err(e) => Err(e),
    }
}

fn truncate_readme(readme: &str, max_chars: usize) -> String {
    let truncated = readme.chars().take(max_chars).collect::<String>();
    if readme.chars().count() > max_chars {
        format!("{truncated}...")
    } else {
        truncated
    }
}
