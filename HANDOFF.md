# Handoff

Date: 2026-06-18

## Goal

Rustopedia should feel meaningfully different from a generic coding chat model:

- strong Rust-first behavior
- pair-programming style
- minimal wasted cycles outside Rust development
- better grounding in the local workspace for `ask`, `review`, and especially `edit`
- make smaller local models actually usable for Rust edits by surrounding them with
  deterministic grounding and retry-with-evidence loops

## Headline Result This Session

A local Rust-trained model (**Strand-Rust-Coder-14B-v1-4bit** on oMLX) went from *unable
to land any edit* to reliably landing a **compiling, idiomatic patch**. The breakthrough
was a new **symbolic edit format**: the model names the item it wants to change
(`@replace struct OpenRouterChatResponse`) and `syn` resolves the span — the model never
transcribes anchor bytes, which was the recurring failure mode (abbreviation,
`pub struct` vs `struct`, malformed SEARCH/REPLACE envelopes).

Last validated run: `--prompt "add a custom Deserialize impl for OpenRouterChatResponse
that tolerates a missing choices field"` → the model emitted `@replace struct
OpenRouterChatResponse` adding `#[serde(default)]`, which resolved `[OK]`, applied in the
scratch worktree, **passed `cargo check`**, EXIT 0, zero retries, ~55s.

## What Landed (all on `origin/main`, pushed)

Working branch was merged to `main`; everything below is on `main` through `8e3d569`:

- `0f1b421` docs.rs grounding in edit mode + best-attempt retry selection
- `0f49134` best-attempt scoring (a parsed-but-blocked patch beats an empty one)
- `21526a1` patch parser tolerates the git-conflict SEARCH/REPLACE dialect
  (`<<<REPLACE` divider, `<<<` / `>>>>>>>` closers) Strand emits
- `9f87e6a` surgical cargo_check retry (shows the model its own prior patch) +
  REPLACE-scoping guards
- `3dbcfef` **Phase 1**: syn-based file skeletons — exact signatures of every top-level
  item incl. **private** ones and `#[derive(...)]`, via proc-macro2 span-locations
- `8b53f3b` / `e52575c` bounded skeleton size + dropped Project Overview in edit mode to
  shrink the prompt
- `7c02953` opt-in prompt-budget guard (`RUSTOPEDIA_MAX_PROMPT_TOKENS`, default 0=off)
- `280abda` **Phase 2**: the symbolic edit format
- `8e3d569` conflicting-impl detector (derive + manual impl → E0119) with a targeted retry

## Current Capabilities

**Symbolic edits** (`src/patch.rs`, applied in `src/scratch.rs`, prompt in
`src/generate_prompt.rs::SYMBOLIC_OUTPUT_REQUIREMENT` — presented as PREFERRED):

- Fence: ` ```patch path=… edit=symbolic ` with `@replace` / `@after` / `@before
  <selector>` operations, each body terminated by a `@@@` line.
- Selectors: `struct N | enum N | union N | type N | trait N | fn n | impl Type |
  impl Trait for Type`. `fn n` falls back to an impl method if no top-level fn matches.
- `resolve_item_span` (patch.rs) reuses the Phase-1 syn pattern to find the item's line
  span. `verify_patch`'s `Patch::Symbolic` arm maps resolution → `AnchorStatus` (a syn
  parse failure → retryable `FileReadError`, with a "fall back to SEARCH/REPLACE"
  directive). Apply is span-based (`line_range_to_byte_range` + `replace_range`/insert,
  descending-order, idempotent inserts).
- Failure directives (`src/retry_loop.rs`): `SymbolicNotFound`/`SymbolicAmbiguous` list
  the file's actual item names (`list_item_selectors`); `SymbolicParseFailed` says fall
  back to SEARCH/REPLACE.
- **SEARCH/REPLACE still fully supported** as the fallback format.

**Conflicting-impl detector** (`patch.rs::detect_symbolic_conflicts`, wired in
`runtime.rs::execute_turn` before scratch validation): when an `@after`/`@before` body is
`impl Trait for Type` and `Type` derives `Trait`, it emits a precise retry telling the
model to add a second `@replace struct Type` op dropping the derive. cargo_check is the
backstop. **Note: committed + unit-tested but not yet exercised end-to-end** — the last
Strand run took the clean `@replace` path and never hit a conflict.

**syn-grounded skeletons** (`runtime.rs::render_file_skeleton_syn`): exact source
signatures of all items (pub + private), capped at 1600 chars/file, 3 files, types-first;
falls back to the old line-scan when a file doesn't parse.

**Edit-mode retry loop** (`runtime.rs::execute_turn` + `retry_loop.rs`) — single
`RUSTOPEDIA_EDIT_MAX_RETRIES` budget across: patch-format drift, anchor mismatch,
symbolic not-found/ambiguous, symbolic-conflict, and cargo_check failure. **Best-attempt
selection** (`attempt_quality`) emits the highest-scoring iteration, so a degraded retry
never overwrites a good earlier patch.

## Infrastructure / Environment Notes

- **oMLX prefill wall (resolved).** The server rejected prompts >~3.5K tokens with a 500
  ("Prefill context too large") due to a mispredicting guard. Fix was server-side, in
  `~/.omlx/settings.json` + a kernel limit: `sudo sysctl iogpu.wired_limit_mb=30720`
  (resets on reboot — re-run it) and `scheduler.chunked_prefill: true` (needs an oMLX
  restart). With these, ~5K-token edit prompts prefill fine. The guard knobs
  (`memory.prefill_memory_guard`, `memory_guard_tier`, `prefill_safe_zone_ratio`) are
  there if it recurs.
- `.env`: edit model is `Strand-Rust-Coder-14B-v1-4bit` (`RUSTOPEDIA_EDIT_MODEL_NAME`);
  ask/review stay on `Qwen3.5-9B-MLX-4bit`. Provider `openrouter` → local
  `http://127.0.0.1:8000/v1`, key `9727`. `RUSTOPEDIA_MAX_PROMPT_TOKENS` is commented out
  (the server fix removed the need to cap).

## Next Likely Work

1. **Exercise the conflicting-impl detector e2e.** Get a run where Strand takes the
   `@after` + manual-impl route (vs the clean `@replace`) and confirm the `kind=
   symbolic_conflict` retry steers it to a two-op edit that compiles. Sampling variance —
   may need several runs.
2. **Fix the flaky scratch tests.** `src/scratch.rs` git-worktree tests flake under
   parallel `cargo test` (transient `git worktree add` contention); all pass in isolation
   / `--test-threads=1`. Add a serialization guard (e.g. a shared mutex) so CI is green.
3. **Measure symbolic-edit success rate.** Now that Strand is unblocked, run the edit
   prompt N× to quantify: format-adoption rate, applicable rate, clean-compile rate.
4. **Exercise `@before` and the `fn`→impl-method selector** — implemented, lightly tested.
5. Optional: field-level symbolic ops (`@add-attr`, `@add-field`) if `@replace`-whole-item
   proves too coarse; tree-sitter only if we need error-tolerant parsing of broken files.

## Verification Status

- `cargo build` clean (one pre-existing `ParsedPatches::is_empty` dead_code warning).
- `cargo clippy --all-targets --no-deps` clean for new code (pre-existing
  `too_many_arguments` + `sort_by_key` warnings remain in older files).
- `cargo test` — **119 passing** single-threaded (`cargo test -- --test-threads=1`).
  Under default parallelism the scratch git-worktree tests flake (see Next Work #2).
- End-to-end: Strand lands a compiling symbolic edit (EXIT 0) on the OpenRouterChatResponse
  prompt; full transcripts at `/tmp/strand_*.log` from this session.

## How To Run

```bash
cargo build
RUSTOPEDIA_DEBUG=1 ./target/debug/rustopedia --mode edit --max-retries 4 \
  --prompt "add a custom Deserialize impl for OpenRouterChatResponse that tolerates a missing choices field"
```
Watch the debug trace for: `Edit doc-targets selected …`, the synthesized prompt size
(`prompt_tokens~=`), `retry_loop iter=… verified: patches=… edits_ok=…`, `kind=…` retry
scheduling, `emitting best attempt from iteration=…`, and the final `patch_preview`
(`[1] symbolic-edit … : [OK]`). Make sure the oMLX server is up (`curl -s -o /dev/null -w
"%{http_code}" -H "Authorization: Bearer 9727" http://127.0.0.1:8000/v1/models` → 200).

## Repo State

`main` = `origin/main` = `8e3d569`, tree clean. The earlier `refactor-agent-runtime`
branch is merged. The deprecated docs-routing plan and this session's Phase-2 plan live in
`~/.claude/plans/squishy-gliding-wirth.md` (Phase 2 design, for reference).
