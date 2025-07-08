use reqwest::header::USER_AGENT;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct CrateInfo {
    name: String,
    description: String,
    version: String,
    homepage: Option<String>,
}

pub async fn search_crates(query: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("https://crates.io/api/v1/crates?q={}", query);
    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header(USER_AGENT, "Rust-duck/1.0")
        .send()
        .await?;

    let body = response.text().await?;
    let data: serde_json::Value = serde_json::from_str(&body)?;

    let mut results = Vec::new();

    if let Some(crates) = data["crates"].as_array() {
        for krate in crates.iter().take(5) {
            results.push(CrateInfo {
                name: krate["id"].as_str().unwrap_or_default().to_string(),
                description: krate["description"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                version: krate["newest_version"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                homepage: krate["homepage"].as_str().map(|s| s.to_string()),
            });
        }
    }

    Ok(json!({ "results": results }))
}
