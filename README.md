# Rustopedia

Local Rust coding assistant with explicit `ask`, `review`, and `edit` modes.

## Current Architecture
- Rust CLI chat loop in `src/main.rs`
- Runtime modes are managed in `src/runtime.rs`
- `review` mode is local-first and starts from rust-analyzer workspace context
- Planner/tool routing is currently used for non-review flows
- Vector retrieval currently uses Qdrant HTTP (`src/tools/qdrant_client.rs`)
- Query embeddings are still generated via `rag/embed_query.py` for Qdrant-backed retrieval

## Setup

### 1. Rust app
```bash
cargo build
```

### 2. Python environment for embeddings/indexing
This is only needed for the legacy Qdrant embedding/query path.
```bash
cd rag
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
cd ..
```

### 3. Run Qdrant
Use a local Qdrant instance on `http://localhost:6333` (or set `RUSTOPEDIA_QDRANT_URL`).

## Index Documents into Qdrant
Example:
```bash
python3 rag/embed_docs_qdrant.py \
  --dir sample_docs/rust-docs \
  --collection rust-docs
```

Collections used by runtime routing:
- `rust-docs`
- `rust-book`
- `crates`

## Run
```bash
cargo run
```

Useful commands in the chat:

```text
/mode ask
/mode review
/mode edit
/status
/help
```

## Configuration
Environment variables:
- `RUSTOPEDIA_LLM_PROVIDER` (`ollama` or `openrouter`, default: `ollama`)
- `RUSTOPEDIA_MODEL_NAME` (default answer model: `deepseek-coder-v2:latest`)
- `RUSTOPEDIA_PLANNER_MODEL_NAME` (default: same as `RUSTOPEDIA_MODEL_NAME`)
- `RUSTOPEDIA_OLLAMA_BASE_URL` (default: `http://localhost:11434`)
- `RUSTOPEDIA_OPENROUTER_BASE_URL` (default: `https://openrouter.ai/api/v1`)
- `RUSTOPEDIA_OPENROUTER_API_KEY` (required when provider is `openrouter`)
- `RUSTOPEDIA_QDRANT_URL` (default: `http://localhost:6333`)
- `RUSTOPEDIA_RAG_TOP_K` (default: `10`)
- `RUSTOPEDIA_HTTP_CONNECT_TIMEOUT_SECS` (default: `10`)
- `RUSTOPEDIA_HTTP_REQUEST_TIMEOUT_SECS` (default: `30`)
- `RUSTOPEDIA_EMBED_QUERY_TIMEOUT_SECS` (default: `30`)
- `RUSTOPEDIA_RUST_ANALYZER_BIN` (default: `rust-analyzer`)
- `RUSTOPEDIA_RUST_ANALYZER_TIMEOUT_SECS` (default: `20`)
- `RUSTOPEDIA_PYTHON_BIN` (default: `python3`)
- `RUSTOPEDIA_EMBED_QUERY_SCRIPT` (default: `rag/embed_query.py`)
- `RUSTOPEDIA_PROJECT_ROOT` (default: `.`)

## Notes
- `rag/rag_server.py` is a legacy Chroma path and is not used by the current Rust runtime.
- Planner/output parsing and orchestration are being actively cleaned up on branch `improve-routing-cleanup`.
- Startup now validates configuration and exits early with an error if required values are missing.
- `review` mode does not require Qdrant and skips planner/tool routing in favor of local workspace analysis.
