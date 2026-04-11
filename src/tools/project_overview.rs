use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use regex::Regex;

use crate::config::AppConfig;

pub async fn summarize_project(config: &AppConfig) -> Result<String> {
    let project_root = Path::new(&config.project_root);
    let seed_files = discover_seed_files(project_root);
    let module_files = discover_module_files(project_root, &seed_files)?;

    let mut files = BTreeSet::new();
    files.extend(seed_files);
    files.extend(module_files);

    let package = summarize_package(project_root)?;
    let readme = summarize_readme(project_root)?;
    let source_layout = summarize_source_layout(project_root)?;
    let file_roles = summarize_file_roles(project_root, &files)?;

    Ok(format!(
        "Project overview:\n{package}\n{readme}\n{source_layout}\nKey files:\n{file_roles}"
    ))
}

fn discover_seed_files(project_root: &Path) -> BTreeSet<PathBuf> {
    let candidates = [
        "Cargo.toml",
        "README.md",
        "src/main.rs",
        "src/lib.rs",
        "src/runtime.rs",
        "src/session.rs",
        "src/config.rs",
        "src/planner.rs",
        "src/tools/mod.rs",
    ];

    candidates
        .into_iter()
        .map(|path| project_root.join(path))
        .filter(|path| path.exists())
        .collect()
}

fn discover_module_files(project_root: &Path, seed_files: &BTreeSet<PathBuf>) -> Result<BTreeSet<PathBuf>> {
    let module_decl = Regex::new(r"(?m)^\s*(?:pub\s+)?mod\s+([a-zA-Z0-9_]+)\s*;")
        .context("failed to compile module regex")?;
    let mut files = BTreeSet::new();

    for path in seed_files {
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        let content = fs::read_to_string(path).unwrap_or_default();
        let parent = path.parent().unwrap_or(project_root);

        for capture in module_decl.captures_iter(&content) {
            let module = capture.get(1).map(|m| m.as_str()).unwrap_or_default();
            let file_candidate = parent.join(format!("{module}.rs"));
            let mod_candidate = parent.join(module).join("mod.rs");

            if file_candidate.exists() {
                files.insert(file_candidate);
            } else if mod_candidate.exists() {
                files.insert(mod_candidate);
            }
        }
    }

    Ok(files)
}

fn summarize_package(project_root: &Path) -> Result<String> {
    let cargo_toml = fs::read_to_string(project_root.join("Cargo.toml"))
        .context("failed to read Cargo.toml")?;

    let package_name = extract_toml_value(&cargo_toml, "name").unwrap_or_else(|| "unknown".to_string());
    let version = extract_toml_value(&cargo_toml, "version").unwrap_or_else(|| "unknown".to_string());
    let edition = extract_toml_value(&cargo_toml, "edition").unwrap_or_else(|| "unknown".to_string());

    let dependencies = cargo_toml
        .lines()
        .skip_while(|line| line.trim() != "[dependencies]")
        .skip(1)
        .take_while(|line| !line.trim().starts_with('['))
        .filter_map(|line| line.split('=').next().map(str::trim))
        .filter(|name| !name.is_empty())
        .take(8)
        .map(str::to_string)
        .collect::<Vec<_>>();

    let deps = if dependencies.is_empty() {
        "none".to_string()
    } else {
        dependencies.join(", ")
    };

    Ok(format!(
        "Package: {package_name} v{version} (edition {edition})\nPrimary dependencies: {deps}"
    ))
}

fn summarize_readme(project_root: &Path) -> Result<String> {
    let readme_path = project_root.join("README.md");
    if !readme_path.exists() {
        return Ok("README: not present.".to_string());
    }

    let readme = fs::read_to_string(readme_path).context("failed to read README.md")?;
    let summary_lines = readme
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with('#'))
        .take(4)
        .collect::<Vec<_>>();

    if summary_lines.is_empty() {
        Ok("README: present but no summary text extracted.".to_string())
    } else {
        Ok(format!("README summary: {}", summary_lines.join(" ")))
    }
}

fn summarize_source_layout(project_root: &Path) -> Result<String> {
    let src_dir = project_root.join("src");
    if !src_dir.exists() {
        return Ok("Source layout: no src directory found.".to_string());
    }

    let entries = fs::read_dir(src_dir)
        .context("failed to read src directory")?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();

    Ok(format!("Top-level source entries: {}", entries.join(", ")))
}

fn summarize_file_roles(project_root: &Path, files: &BTreeSet<PathBuf>) -> Result<String> {
    let mut summaries = Vec::new();

    for path in files {
        let relative = path
            .strip_prefix(project_root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        let summary = if relative == "Cargo.toml" {
            "crate manifest and dependency configuration".to_string()
        } else if relative == "README.md" {
            "project overview and setup notes".to_string()
        } else {
            summarize_rust_file(path).unwrap_or_else(|_| infer_role_from_path(&relative))
        };

        summaries.push(format!("- {relative}: {summary}"));
    }

    Ok(summaries.join("\n"))
}

fn summarize_rust_file(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;

    if let Some(doc) = first_doc_comment(&content) {
        return Ok(doc);
    }

    if let Some(signature) = first_item_signature(&content) {
        return Ok(signature);
    }

    Ok(infer_role_from_path(&path.to_string_lossy()))
}

fn first_doc_comment(content: &str) -> Option<String> {
    let docs = content
        .lines()
        .map(str::trim)
        .filter_map(|line| line.strip_prefix("//!").or_else(|| line.strip_prefix("///")))
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .take(2)
        .collect::<Vec<_>>();

    if docs.is_empty() {
        None
    } else {
        Some(docs.join(" "))
    }
}

fn first_item_signature(content: &str) -> Option<String> {
    content
        .lines()
        .map(str::trim)
        .find(|line| {
            line.starts_with("pub struct ")
                || line.starts_with("struct ")
                || line.starts_with("pub enum ")
                || line.starts_with("enum ")
                || line.starts_with("pub fn ")
                || line.starts_with("fn ")
                || line.starts_with("pub async fn ")
                || line.starts_with("async fn ")
        })
        .map(|line| format!("contains {}", line.trim_end_matches('{').trim()))
}

fn infer_role_from_path(path: &str) -> String {
    if path.ends_with("main.rs") {
        "CLI entrypoint".to_string()
    } else if path.ends_with("runtime.rs") {
        "mode-aware request pipeline and orchestration".to_string()
    } else if path.ends_with("session.rs") {
        "session state and command parsing".to_string()
    } else if path.ends_with("config.rs") {
        "environment-backed configuration".to_string()
    } else if path.ends_with("planner.rs") {
        "LLM-driven tool routing".to_string()
    } else if path.ends_with("llm.rs") {
        "LLM client integration".to_string()
    } else if path.ends_with("tools/mod.rs") {
        "tool module registry".to_string()
    } else if path.contains("qdrant") {
        "legacy vector retrieval integration".to_string()
    } else if path.contains("rust_analyzer") {
        "local rust-analyzer project analysis".to_string()
    } else if path.contains("docs") {
        "external Rust documentation lookup".to_string()
    } else if path.contains("crate_search") {
        "crates.io search integration".to_string()
    } else if path.contains("github") {
        "GitHub example search integration".to_string()
    } else {
        "Rust source file".to_string()
    }
}

fn extract_toml_value(toml: &str, key: &str) -> Option<String> {
    let prefix = format!("{key} =");
    toml.lines()
        .map(str::trim)
        .find(|line| line.starts_with(&prefix))
        .and_then(|line| line.split('=').nth(1))
        .map(str::trim)
        .map(|value| value.trim_matches('"').to_string())
}
