# Improvement Plan

## Phase 1: Stabilize Core Runtime
- [x] Add central config for model, provider, vector DB URL, and top-k.
- [x] Add central config for HTTP/client timeouts.
- [ ] Remove hardcoded absolute paths and shell-interpolated commands in embedding/query flow.
- [ ] Normalize errors across planner, tools, and generation path (no silent `None` failures).
- [ ] Align docs with actual project structure and runnable commands.

## Phase 2: Routing + Tooling Quality
- [ ] Replace regex-based plan parsing with strict typed JSON schema.
- [ ] Add deterministic fallback routes when planner output is invalid or low-confidence.
- [ ] Split orchestration into explicit stages: `route -> execute -> synthesize`.
- [ ] Improve tool response quality and HTTP handling (status checks, retries, rate-limit handling).

## Phase 3: Chat Capabilities
- [ ] Add `ask` mode (Q&A only, no file edits).
- [ ] Add `edit` mode (read/write/fix code directly with patch output + summary).
- [ ] Add guardrails for `edit` mode (confirm intent, show touched files, run checks when possible).
- [ ] Track mode in session state and expose a clear user command to switch modes.

## Phase 4: Provider + Evaluation
- [ ] Add provider abstraction (Ollama first, OpenRouter HTTP client second).
- [ ] Support separate models for planning and final answer synthesis.
- [ ] Build an eval set (20-50 Rust tasks) to measure routing quality, answer quality, and latency.
- [ ] Add automated checks for regression across routing and tool invocation.
