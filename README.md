# Rustopedia

Local Rust coding assistant with tool routing and RAG over Rust documentation.

## Current Architecture
- Rust CLI chat loop in `src/main.rs`
- Planner model chooses tools (`crate_agent`, `docs_agent`, `github_agent`)
- Tool outputs + RAG context are merged into a final prompt
- Vector retrieval currently uses Qdrant HTTP (`src/tools/qdrant_client.rs`)
- Query embeddings are generated via `rag/embed_query.py`

## Setup

### 1. Rust app
```bash
cargo build
```

### 2. Python environment for embeddings/indexing
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

## Configuration
Environment variables:
- `RUSTOPEDIA_LLM_PROVIDER` (`ollama` or `openrouter`, default: `ollama`)
- `RUSTOPEDIA_MODEL_NAME` (default answer model: `openhermes`)
- `RUSTOPEDIA_PLANNER_MODEL_NAME` (default: same as `RUSTOPEDIA_MODEL_NAME`)
- `RUSTOPEDIA_OLLAMA_BASE_URL` (default: `http://localhost:11434`)
- `RUSTOPEDIA_OPENROUTER_BASE_URL` (default: `https://openrouter.ai/api/v1`)
- `RUSTOPEDIA_OPENROUTER_API_KEY` (required when provider is `openrouter`)
- `RUSTOPEDIA_QDRANT_URL` (default: `http://localhost:6333`)
- `RUSTOPEDIA_RAG_TOP_K` (default: `10`)
- `RUSTOPEDIA_PYTHON_BIN` (default: `python3`)
- `RUSTOPEDIA_EMBED_QUERY_SCRIPT` (default: `rag/embed_query.py`)
- `RUSTOPEDIA_PROJECT_ROOT` (default: `.`)

## Notes
- `rag/rag_server.py` is a legacy Chroma path and is not used by the current Rust runtime.
- Planner/output parsing and orchestration are being actively cleaned up on branch `improve-routing-cleanup`.
- Startup now validates configuration and exits early with an error if required values are missing.
