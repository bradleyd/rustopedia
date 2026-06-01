# Rustopedia

Local Rust coding assistant with explicit `ask`, `review`, and `edit` modes. Designed to make smaller local models usable for real Rust work by surrounding them with deterministic grounding, retries with concrete failure evidence, and a safe scratch-apply path.

## Architecture

- Rust CLI + REPL in `src/main.rs`; non-interactive one-shot path in `src/cli.rs`.
- Runtime modes managed in `src/runtime.rs`; routing is Rust-intent-first rather than planner-driven.
- `review` and `edit` are local-first and prioritize workspace/compiler/lint/test evidence.
- Edit mode grounds the model with deterministic context before generation:
  - `Likely Edit Targets` from `src/tools/edit_context.rs`
  - `Current Code Facts` and `File Skeletons` extractors in `src/runtime.rs` — list every existing struct/enum/field/fn/env var in the touched files so the model stops inventing absent symbols
  - workspace diff, `cargo check`, `cargo clippy`, `cargo test` summaries
- Edit-mode retry loop (`src/retry_loop.rs`) handles three failure classes, sharing a single `RUSTOPEDIA_EDIT_MAX_RETRIES` budget:
  1. **Patch-format drift** — response had no parseable patch blocks. Directive inlines the canonical SEARCH/REPLACE shape. Circuit-breaks on two consecutive format failures.
  2. **Anchor mismatch** — SEARCH text not found, ambiguous, or `new=true` on an existing file. Directive shows the real file slice around the model's intended anchor.
  3. **Validation failure** — patches applied cleanly in a scratch git worktree (`src/scratch.rs`) but `cargo check` failed there. Directive shows the compiler error and a post-patch file slice around each failing span.
- Vector retrieval is optional and uses Qdrant HTTP (`src/tools/qdrant_client.rs`); only required for legacy semantic retrieval. Query embeddings come from `rag/embed_query.py`.
- Mode contracts and example prompts live in `MODES.md`. Prompt/context layering design lives in `MEMORY_MAPPER.md`.

## Quick start

```bash
cargo build
```

Set provider credentials (see [Configuration](#configuration)). The minimum is `RUSTOPEDIA_LLM_PROVIDER` plus the relevant model and API-key/URL env vars.

### REPL

```bash
cargo run
```

Commands available in the chat:

```text
/mode ask       Rust-first Q&A
/mode review    inspect or critique the local workspace (no edits)
/mode edit      plan and emit Rust code changes
/status         current mode + last-turn memory summary
/memory         detailed view of last-turn memory layers
/help           list commands
exit            quit
```

### CLI (non-interactive)

When any argument is passed, the binary runs a single query and exits with a structured outcome — designed for scripted evaluation of prompt/model/config changes without REPL friction.

```bash
rustopedia --mode edit --prompt "fix the hard coded paths for openrouter"
rustopedia --mode ask  --prompt "what does the planner do?"
echo "fix the openrouter url" | rustopedia --mode edit
rustopedia --mode edit --prompt "..." --json --no-banner       # structured output
rustopedia --mode edit --prompt "..." --max-retries 4          # override retry budget
```

Flags:

```text
--mode <ask|review|edit>   required
--prompt <text>            prompt text (or read from stdin if omitted)
--json                     emit a single JSON record describing the outcome
--max-retries <n>          override RUSTOPEDIA_EDIT_MAX_RETRIES for this run
--no-banner                suppress the startup banner in pretty-text output
```

Exit codes:

```text
0  query completed cleanly
1  edit mode: anchor failures remain after retries
2  edit mode: no patches were emitted
3  config/setup error or request failure
```

Today the CLI never writes patches to disk — it always prints the proposed patch blocks and preview. Apply them yourself, or pipe the `--json` output into your own apply step.

## Configuration

Environment variables:

| Variable | Default | Notes |
|---|---|---|
| `RUSTOPEDIA_LLM_PROVIDER` | `ollama` | `ollama` or `openrouter` |
| `RUSTOPEDIA_MODEL_NAME` | `deepseek-coder-v2:latest` | Default answer model |
| `RUSTOPEDIA_REVIEW_MODEL_NAME` | same as `RUSTOPEDIA_MODEL_NAME` | |
| `RUSTOPEDIA_EDIT_MODEL_NAME` | same as `RUSTOPEDIA_MODEL_NAME` | |
| `RUSTOPEDIA_OLLAMA_BASE_URL` | `http://localhost:11434` | |
| `RUSTOPEDIA_OPENROUTER_BASE_URL` | `https://openrouter.ai/api/v1` | |
| `RUSTOPEDIA_OPENROUTER_API_KEY` | — | Required when provider is `openrouter` |
| `RUSTOPEDIA_LLM_MAX_TOKENS` | provider default | Cap on generation length |
| `RUSTOPEDIA_EDIT_MAX_RETRIES` | `2` | Shared budget across format / anchor / cargo-check retries |
| `RUSTOPEDIA_CARGO_TIMEOUT_SECS` | provider default | Per-invocation cargo timeout |
| `RUSTOPEDIA_RUST_ANALYZER_BIN` | `rust-analyzer` | |
| `RUSTOPEDIA_RUST_ANALYZER_TIMEOUT_SECS` | `20` | |
| `RUSTOPEDIA_RIPGREP_BIN` | `rg` | |
| `RUSTOPEDIA_HTTP_CONNECT_TIMEOUT_SECS` | `10` | |
| `RUSTOPEDIA_HTTP_REQUEST_TIMEOUT_SECS` | `30` | |
| `RUSTOPEDIA_PROJECT_ROOT` | `.` | Tree the runtime analyzes and validates against |
| `RUSTOPEDIA_DEBUG` | unset | Any value enables `[debug]` stage and retry-loop logging |
| `RUSTOPEDIA_QDRANT_URL` | `http://localhost:6333` | Only needed for the optional Qdrant retrieval path |
| `RUSTOPEDIA_RAG_TOP_K` | `10` | |
| `RUSTOPEDIA_EMBED_QUERY_TIMEOUT_SECS` | `30` | |
| `RUSTOPEDIA_PYTHON_BIN` | `python3` | |
| `RUSTOPEDIA_EMBED_QUERY_SCRIPT` | `rag/embed_query.py` | |

Local environment overrides can go in `.env` (gitignored). Startup validates configuration and exits early if required values are missing.

## Optional: vector retrieval (Qdrant)

Only needed for legacy semantic retrieval — `ask`, `review`, and `edit` all work without it. To enable:

```bash
cd rag
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
cd ..
```

Run a local Qdrant instance on `http://localhost:6333` (or set `RUSTOPEDIA_QDRANT_URL`). Index documents:

```bash
python3 rag/embed_docs_qdrant.py \
  --dir sample_docs/rust-docs \
  --collection rust-docs
```

Collections used by runtime routing: `rust-docs`, `rust-book`, `crates`.

## Notes

- `rag/rag_server.py` is a legacy Chroma path and is not used by the current Rust runtime.
- External crate/docs/example lookup is a fallback for Rust questions, not the default runtime path.
- `review` mode does not require Qdrant and stays local to the Rust workspace analysis path.
- The scratch worktree used by validation retry inherits your uncommitted working state (tracked diff + untracked files) so it validates the code you're actually editing, not just `HEAD`. It is removed on drop.
