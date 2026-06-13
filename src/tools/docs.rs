use scraper::{Html, Selector};

use crate::config::AppConfig;

fn extract_docs_as_json(html: &str) -> serde_json::Value {
    let document = Html::parse_document(html);
    let main_selector = Selector::parse("main").unwrap();
    let mut docs = Vec::new();

    if let Some(main_element) = document.select(&main_selector).next() {
        let content_selector = Selector::parse("p, pre").unwrap();
        for element in main_element.select(&content_selector) {
            let name = element.value().name();
            let text = element.text().collect::<String>().trim().to_string();

            if text.is_empty() {
                continue;
            }

            let entry = if name == "pre" {
                serde_json::json!({
                    "type": "code",
                    "content": text
                })
            } else {
                // "p"
                serde_json::json!({
                    "type": "paragraph",
                    "content": text
                })
            };
            docs.push(entry);
            if docs.len() >= MAX_DOC_ENTRIES {
                break;
            }
        }
    }

    serde_json::Value::Array(docs)
}

/// Cap on doc paragraphs/code blocks returned, to keep prompts small for local models.
const MAX_DOC_ENTRIES: usize = 25;

/// Build ordered candidate URLs for the *specific item page* an input names, e.g.
/// `serde::Deserialize` → `.../serde/trait.Deserialize.html`. docs.rs/doc.rust-lang.org
/// item pages are `{kind}.{Name}.html`; since the input doesn't carry the item kind, we
/// try the plausible kinds for the leaf's casing in order and let the caller take the
/// first that returns HTTP 200. Returns empty for a bare crate name (no `::` path).
fn item_url_candidates(input: &str) -> Vec<String> {
    let parts: Vec<&str> = input
        .split("::")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    if parts.len() < 2 {
        return Vec::new();
    }

    let crate_name = parts[0];
    let leaf = *parts.last().unwrap();
    let middle = &parts[1..parts.len() - 1];

    let base = if ["std", "core", "alloc"].contains(&crate_name) {
        format!("https://doc.rust-lang.org/{crate_name}")
    } else {
        format!("https://docs.rs/{crate_name}/latest/{crate_name}")
    };
    let dir = if middle.is_empty() {
        base
    } else {
        format!("{base}/{}", middle.join("/"))
    };

    let pascal = leaf.chars().next().is_some_and(|c| c.is_ascii_uppercase());
    let mut candidates = Vec::new();
    if pascal {
        for kind in ["trait", "struct", "enum", "type"] {
            candidates.push(format!("{dir}/{kind}.{leaf}.html"));
        }
    } else {
        for kind in ["fn", "macro"] {
            candidates.push(format!("{dir}/{kind}.{leaf}.html"));
        }
        // A lowercase leaf may instead be a submodule.
        candidates.push(format!("{dir}/{leaf}/index.html"));
        candidates.push(format!("{dir}/constant.{leaf}.html"));
    }
    candidates
}

fn build_url(crate_input: &str) -> String {
    if crate_input.contains("::") {
        let parts: Vec<&str> = crate_input.split("::").collect();
        let crate_base = parts[0];
        let last = parts.last().unwrap_or(&"");

        if ["std", "core", "alloc"].contains(&crate_base) {
            if last
                .chars()
                .next()
                .map(|c| c.is_lowercase())
                .unwrap_or(true)
            {
                format!(
                    "https://doc.rust-lang.org/{}/{}/index.html",
                    crate_base,
                    parts[1..].join("/")
                )
            } else {
                format!(
                    "https://doc.rust-lang.org/{}/{}/struct.{}.html",
                    crate_base,
                    parts[1..parts.len() - 1].join("/"),
                    last
                )
            }
        } else {
            format!(
                "https://docs.rs/{}/latest/{}/?search={}",
                crate_base,
                crate_base,
                parts[1..].join("::")
            )
        }
    } else {
        format!("https://docs.rs/{}/latest/{}/", crate_input, crate_input)
    }
}

pub async fn search_docs(
    input: &str,
    config: &AppConfig,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = config.build_http_client()?;

    // Prefer the specific item page (real signature + examples) over the crate-root
    // search page, which is rendered client-side and scrapes to a generic overview.
    for url in item_url_candidates(input) {
        let Ok(resp) = client.get(&url).send().await else {
            continue;
        };
        if !resp.status().is_success() {
            continue;
        }
        let body = resp.text().await?;
        let docs = extract_docs_as_json(&body);
        // A 200 page with no extractable prose (e.g. a stub) shouldn't win over the
        // fallback; only return when we actually scraped something.
        if docs.as_array().is_some_and(|d| !d.is_empty()) {
            return Ok(serde_json::json!({
                "input": input,
                "url": url,
                "docs": docs
            }));
        }
    }

    // Fallback: crate-root / search page (handles bare crate names and re-exports whose
    // canonical item page we can't address).
    let url = build_url(input);
    let body = client.get(&url).send().await?.text().await?;
    let docs = extract_docs_as_json(&body);

    Ok(serde_json::json!({
        "input": input,
        "url": url,
        "docs": docs
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pascal_leaf_tries_type_kinds_in_order() {
        assert_eq!(
            item_url_candidates("serde::Deserialize"),
            vec![
                "https://docs.rs/serde/latest/serde/trait.Deserialize.html",
                "https://docs.rs/serde/latest/serde/struct.Deserialize.html",
                "https://docs.rs/serde/latest/serde/enum.Deserialize.html",
                "https://docs.rs/serde/latest/serde/type.Deserialize.html",
            ]
        );
    }

    #[test]
    fn nested_module_path_becomes_directories() {
        assert_eq!(
            item_url_candidates("tokio::sync::Mutex")[1],
            "https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html"
        );
    }

    #[test]
    fn std_crate_routes_to_rust_lang_org() {
        assert_eq!(
            item_url_candidates("std::collections::HashMap")[1],
            "https://doc.rust-lang.org/std/collections/struct.HashMap.html"
        );
    }

    #[test]
    fn lowercase_leaf_tries_fn_macro_module() {
        assert_eq!(
            item_url_candidates("tokio::spawn"),
            vec![
                "https://docs.rs/tokio/latest/tokio/fn.spawn.html",
                "https://docs.rs/tokio/latest/tokio/macro.spawn.html",
                "https://docs.rs/tokio/latest/tokio/spawn/index.html",
                "https://docs.rs/tokio/latest/tokio/constant.spawn.html",
            ]
        );
    }

    #[test]
    fn bare_crate_has_no_item_candidates() {
        assert!(item_url_candidates("tokio").is_empty());
    }
}
