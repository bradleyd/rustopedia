# Handoff

Date: 2026-06-01

## Goal

Rustopedia should feel meaningfully different from a generic coding chat model:

- strong Rust-first behavior
- pair-programming style
- minimal wasted cycles outside Rust development
- better grounding in the local workspace for `ask`, `review`, and especially `edit`
- make smaller local models actually usable for Rust edits by surrounding them with deterministic grounding and retry-with-evidence loops

## Where We Left Off

Three commits on `refactor-agent-runtime` covered this session's work:

- `471b128` — CLI one-shot, worktree validation retry, file skeleton grounding
- `4a64da8` — Format-drift retry with circuit breaker
- (README refresh staged but not yet committed at end of session)

End-to-end smoke against Qwen3.5-9B (openrouter) now shows all three retry kinds firing in sequence on a single hard prompt: format → cargo_check → (anchor on third iter). Iter 0 went from a 178s soliloquy to a 17s clean parseable patch (10x speedup) once the format-reset directive landed.

## Current Capabilities

**Edit-mode retry loop** (`src/retry_loop.rs`, wired in `src/runtime.rs::execute_turn`) — shares a single `RUSTOPEDIA_EDIT_MAX_RETRIES` budget across three failure classes, evaluated in priority order:

1. **Patch-format drift** (`PatchFormatError`) — fires when no parseable patch blocks. Directive inlines the canonical SEARCH/REPLACE shape and bounds the model's previous output excerpt to 800 chars. Circuit-breaks on two consecutive format failures.
2. **Anchor mismatch** (`AnchorLineFound` / `AnchorLineMissing` / `MultipleMatches` / `CreateOnExistingFile`) — directive shows the real file slice around the model's intended anchor.
3. **Validation failure** (`CargoCheckFailed`) — patches apply cleanly in a scratch git worktree (`src/scratch.rs`), then `cargo check --message-format=short` runs there with `CARGO_TARGET_DIR` shared with the main project for incremental compilation. Directive shows the compiler error and post-patch file slices around each failing span.

**Scratch overlay** (`src/scratch.rs::ScratchOverlay`):

- `git worktree add --detach HEAD` into a temp dir
- mirrors uncommitted tracked changes via `git diff HEAD | git apply`
- copies untracked-but-not-ignored files
- tolerates duplicate patches idempotently (skips when SEARCH is gone but REPLACE is already present)
- removed via `git worktree remove --force` on Drop

**Grounding extractors** (`src/runtime.rs`):

- `current_code_facts` — names of every struct/enum/fn/field/env var present in working-memory excerpts (existed before this session)
- `file_skeletons` — for each file referenced by an excerpt, re-reads the **full file** from disk and emits a compact skeleton of top-level structs (with field names + types), enums (with variant names), and pub fn signatures. This fixed the paralysis failure mode where the model refused to act because only a windowed excerpt was visible.

**Non-interactive CLI** (`src/cli.rs`, wired in `src/main.rs`):

- `rustopedia --mode <ask|review|edit> --prompt "..."`
- stdin if `--prompt` omitted
- `--json` for structured output (`OneShotReport`)
- `--max-retries N` overrides env default
- exit codes: 0 clean, 1 anchor failures, 2 no patches emitted (edit), 3 setup error
- REPL behavior unchanged when no args are passed

**Repo hygiene**: `.env` now in `.gitignore`.

## Next Likely Work

**Top priority — fix tool routing in edit mode.** Every smoke run this session printed `"Rust-first routing selected no external tools."` for edit-mode prompts. The crate/docs/github tools exist (`src/tools/{crate_search,docs,github}.rs`) and are referenced in `runtime.rs::run_agent`, but the planner is selecting nothing for edit. Hypothesis: external tool routing was cut during the runtime refactor and never wired back for edit mode. Likely a 1-hour fix once we find where the planner short-circuits. **This is the single highest-leverage next move** — small models benefit hugely from authoritative API examples (docs.rs lookups) at generation time, and we already have the lookup tools built. Investigate first; don't build new retrieval until you know what's actually wired.

**Source-strategy framing** (once routing is unblocked):

- **docs.rs HTML** for any named crate — highest-value, authoritative, always current. Best edit-mode signal.
- **crates.io API** for "what crate solves X" — `ask`-mode concern, less leverage in edit.
- **Local RAG over rust-book / std-docs corpus** — useful for language/idiom questions (lifetimes, traits) where the answer is stable. Existing Qdrant pipeline can serve this if revived.
- **GitHub code search** — last resort: slow and noisy. Useful only when docs.rs doesn't have an example.

Don't pick RAG vs. web search as a binary; pick the *signal* ("show the model a concrete example of the API it's about to call") and route each source to the case where it's strongest.

**Smaller follow-ups identified during this session:**

- **`RUSTOPEDIA_EDIT_MAX_RETRIES=2` is tight** on hard prompts. The Qwen3.5-9B openrouter run needed all 3 attempts and still didn't fully land. Try `--max-retries 4` on representative prompts before tuning the default.
- **Iter 0 cost is still ~3 minutes** for the soliloquy case before format retry can do its work. Retry directives don't help iter 0 — only a system-prompt-level tightening ("emit patches first, prose minimal") can shrink it. Worth a focused experiment.
- **Scratch overlay first `cargo check` is ~16s** even with shared `CARGO_TARGET_DIR` (cargo's per-workspace metadata stamping). If retry latency becomes a complaint, cache one overlay per session instead of per-retry.
- **The `dead_code` warning on `ParsedPatches::is_empty`** in `src/patch.rs:62` was pre-existing and not in scope this session. Either delete or use.
- **`append_edit_outputs` has 9 args** (clippy `too_many_arguments` warning). Pre-existing; bundle into a struct when next touched.

## Verification Completed

After the format-retry change:

- `cargo check` — clean except the pre-existing `ParsedPatches::is_empty` dead_code warning
- `cargo clippy --all-targets --no-deps` — clean except the pre-existing `too_many_arguments` warning
- `cargo test` — 70 passing, 1 ignored, 0 failing (added 2 new directive-rendering tests + 1 skeleton extractor test + 2 scratch overlay tests this session)
- End-to-end smoke against Qwen3.5-9B confirmed format → cargo_check retry handoff works on a real prompt

## Known Open Questions (carried forward)

- whether long sessions need more aggressive memory compaction / session summarization
- whether `project_overview()` is still too large/noisy for smaller local models
- whether `--apply` belongs in the CLI (currently every run is implicitly dry-run because nothing writes patches to disk in production code paths)
- whether a memory view such as `/status` or `/memory` should expose more detail about memory layers and freshness

## First Test To Run After Reboot

If routing-fix work has not started, baseline first:

```bash
RUSTOPEDIA_DEBUG=1 ./target/debug/rustopedia \
  --mode edit \
  --prompt "fix the hard coded paths for openrouter" \
  --json 2>&1 | grep -E "^\[debug\]|^Stage:"
```

Confirm the trace still shows `"Rust-first routing selected no external tools."` for edit-mode. If it does, that's the starting point for the next session — search `src/planner.rs` and `src/runtime.rs::route_tools` for where edit-mode routing decisions are made and why no tool ever fires.

## Repo State

Branch: `refactor-agent-runtime`. Two commits ahead of where this session started, plus a staged-but-uncommitted README refresh. `.env` is now gitignored. Tree is otherwise clean.
