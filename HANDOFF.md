# Handoff

Date: 2026-04-22

## Goal

Rustopedia should feel meaningfully different from a generic coding chat model:

- strong Rust-first behavior
- pair-programming style
- minimal wasted cycles outside Rust development
- better grounding in the local workspace for `ask`, `review`, and especially `edit`

## Where We Left Off

Most recent completed change:

- Added a deterministic edit-mode grounding layer so the model gets explicit `Current Code Facts` extracted from fresh file excerpts before answering.

Implemented in:

- `src/runtime.rs`
- `src/generate_prompt.rs`

What that change does:

- in edit mode, after `Likely Edit Targets` are gathered, the runtime now extracts existing:
  - structs
  - enums
  - functions
  - fields
  - env var names
- these are inserted into working memory as `Current Code Facts`
- edit-memory prioritization now keeps that fact block near the top
- the edit prompt now explicitly treats `Current Code Facts` as authoritative and tells the model not to invent replacements for items already present

## Verification Completed

These passed after the `Current Code Facts` change:

- `cargo fmt`
- `cargo check --quiet`
- `cargo clippy --all-targets --no-deps --quiet`
- `cargo test --quiet`

## Important Current Behavior

Recent harness direction:

- ask mode now uses targeted local excerpts for workspace questions
- edit mode gathers:
  - likely edit targets
  - local file excerpts
  - workspace diff summary
  - cargo check/clippy/test summaries
  - project overview
- vague edit requests clarify only after analysis and only when there is no concrete evidence to act on
- `rust-analyzer` is no longer run by default for broad edit requests

## Main Problem We Were Fixing

Even with relevant excerpts in memory, edit mode was still hallucinating things like:

- fake `Config` structs
- fake `openrouter_path` fields
- made-up replacement APIs

The new deterministic fact block was added specifically to reduce that failure mode.

## First Test To Run After Reboot

Re-test this exact flow:

1. `/mode edit`
2. `lets fix the hard coded paths for openrouter`
3. `/memory`

What to check:

- `/memory` should show `text:Current Code Facts` near the top
- the answer should refer to existing code such as:
  - `AppConfig`
  - `openrouter_base_url`
  - `openrouter_api_key`
  - `generate_openrouter`
- it should stop inventing replacement config structs or fields

## Next Likely Work

If the edit response is still too loose, the next step should be a validator/regeneration pass instead of more prompt wording.

Recommended next improvements:

1. Add a post-generation hallucination check for edit mode.
2. Reject or regenerate when the answer proposes symbols not present in `Current Code Facts` or excerpts.
3. Keep improving local excerpt targeting so domain files beat harness files unless the query is explicitly about the harness.
4. Continue into first-class edit tooling after grounding is stable.

## Known Open Questions

- whether long sessions need more aggressive memory compaction / session summarization
- whether `project_overview()` is still too large/noisy for smaller local models
- whether future file operations should stay shell-driven or move toward first-class tools
- whether a memory view such as `/status` or `/memory` should expose more detail about memory layers and freshness

## Repo State

The worktree already contains many in-progress changes across runtime, prompts, tools, memory, and session handling. Do not assume a clean branch when resuming.
