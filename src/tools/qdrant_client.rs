use serde_json::{Value, json};
use std::error::Error;
use std::time::Duration;

pub struct QdrantClient {
    client: reqwest::Client,
    base_url: String,
}

impl QdrantClient {
    pub fn new(base_url: &str, client: reqwest::Client) -> Self {
        Self { client, base_url: base_url.to_string() }
    }

    pub async fn search_collection(
        &self,
        collection_name: &str,
        query_vector: Vec<f32>,
        top: usize,
    ) -> Result<Vec<SearchResult>, Box<dyn Error>> {
        let search_payload = json!({
            "vector": query_vector,
            "top": top,
            "with_payload": true
        });

        let response = self
            .client
            .post(format!(
                "{}/collections/{}/points/search",
                self.base_url, collection_name
            ))
            .json(&search_payload)
            .send()
            .await?
            .error_for_status()?;

        let search_results: Value = response.json().await?;

        let mut results = Vec::new();
        if let Some(result_array) = search_results["result"].as_array() {
            for result in result_array {
                if let Some(payload) = result["payload"].as_object() {
                    let content = payload
                        .get("text")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();

                    let score = result["score"].as_f64().unwrap_or(0.0);

                    results.push(SearchResult {
                        content,
                        score,
                        metadata: payload.clone(),
                    });
                }
            }
        }

        Ok(results)
    }

    pub async fn list_collections(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}/collections", self.base_url))
            .send()
            .await?;
        let collections: Value = response.json().await?;

        let mut collection_names = Vec::new();
        if let Some(result) = collections["result"].as_object() {
            if let Some(collections_array) = result["collections"].as_array() {
                for collection in collections_array {
                    if let Some(name) = collection["name"].as_str() {
                        collection_names.push(name.to_string());
                    }
                }
            }
        }

        Ok(collection_names)
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub content: String,
    pub score: f64,
    pub metadata: serde_json::Map<String, Value>,
}

// Main function for RAG queries - similar to your current call_rag function
pub async fn query_qdrant(
    collection_name: &str,
    query_vector: Vec<f32>,
    top: usize,
) -> Result<String, Box<dyn Error>> {
    let config = crate::config::AppConfig::from_env();
    let client = QdrantClient::new(&config.qdrant_url, config.build_http_client()?);

    let results = client
        .search_collection(collection_name, query_vector, top)
        .await?;

    // Format results similar to current approach
    let context = results
        .into_iter()
        .map(|result| result.content)
        .collect::<Vec<_>>()
        .join("\n\n");

    Ok(context)
}

// For testing with text queries - generates embeddings using Python script
pub async fn query_qdrant_with_text(
    collection_name: &str,
    query_text: &str,
    top: usize,
) -> Result<String, Box<dyn Error>> {
    // Generate embedding using Python script
    let embedding = generate_embedding(query_text).await?;
    query_qdrant(collection_name, embedding, top).await
}

// Helper function to generate embeddings using Python script
async fn generate_embedding(text: &str) -> Result<Vec<f32>, Box<dyn Error>> {
    use std::process::Command;

    let config = crate::config::AppConfig::from_env();
    let text_owned = text.to_string();
    let timeout = config.embed_query_timeout();
    let output = tokio::time::timeout(timeout, tokio::task::spawn_blocking(move || {
        Command::new(&config.python_bin)
            .arg(&config.embed_query_script)
            .arg(text_owned)
            .current_dir(&config.project_root)
            .output()
    }))
    .await
    .map_err(|_| format!("embedding script timed out after {}", format_duration(timeout)))???;

    if !output.status.success() {
        return Err(format!(
            "Python embedding script failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let embedding_json = String::from_utf8(output.stdout)?;
    let embedding: Vec<f32> = serde_json::from_str(&embedding_json)?;

    Ok(embedding)
}

fn format_duration(duration: Duration) -> String {
    format!("{}s", duration.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_qdrant_search() {
        let config = crate::config::AppConfig::from_env();
        let client = QdrantClient::new(
            &config.qdrant_url,
            config.build_http_client().expect("http client"),
        );

        // Test with dummy vector
        let dummy_vector = vec![0.1; 384];

        match client.search_collection("rust-test", dummy_vector, 5).await {
            Ok(results) => {
                println!("✅ Qdrant search works! Found {} results", results.len());
                for result in results {
                    println!("Content: {}, Score: {}", result.content, result.score);
                }
            }
            Err(e) => {
                println!("❌ Qdrant search failed: {}", e);
            }
        }
    }
}
