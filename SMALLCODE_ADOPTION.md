# Smallcode Adoption: Design Notes for Rustopedia

This document captures which ideas from [smallcode](https://github.com/Doorman11991/smallcode)
are worth adopting in Rustopedia, how each one fits the current architecture,
and the concrete code-level changes required to land them. It is a planning
document, not a spec — modules, env var names, and signatures shown are
proposals to align on before implementation.

BoneScript and MarrowScript are explicitly out of scope: Rustopedia is not in
the business of generating backend projects from `.bone` files or compiling
declarative `.marrow` files into TypeScript. Both target use cases outside
"Rust pair programmer for the local workspace."

## 0. Why borrow from smallcode at all

Rustopedia and smallcode share the same hard constraint: get useful work out
of small, locally-runnable models that drift, truncate, and hallucinate when
pushed past their comfort zone. Smallcode's nine highlights are essentially a
toolbox for making small models reliable; most of them map cleanly onto gaps
that already exist in Rustopedia's pipeline.

Where they don't map, it's usually because Rustopedia is currently
single-shot (one prompt build, one LLM call, one response — see
`src/runtime.rs:175` `execute_turn`). Several smallcode features only pay off
inside a multi-turn agent loop, so this doc separates "drop in now" from
"requires the loop first."

## 1. Context Budget Engine

### Smallcode
> Tool results capped at 4k chars, mid-turn eviction drops old results when
> context grows too large, and semantic compression summarizes history
> instead of dropping it.

### Rustopedia today
The prompt is built by `build_prompt` in `src/generate_prompt.rs:8`, which
concatenates every `WorkingMemoryItem` in `MemoryState::working_memory_text`
(`src/memory.rs:41`) unconditionally. There is debug logging of
`context_chars` and a 4-char-per-token estimate (`src/runtime.rs:666`), but
no actual cap and no eviction. A heavy `cargo test` summary plus a long
project overview can blow past Ollama's default context window for
deepseek-coder-v2 without warning.

`trimmed_history` in `src/runtime.rs:1083` already does the right shape of
thing for conversation history (keep last N turns, char-budget,
truncation marker), but nothing equivalent exists for working memory.

### Fit
Strong fit. This is the cheapest high-value change in the list and unblocks
several others (model profiles set the budget; escalation needs an
"overflowed" signal).

### Proposed changes
- New module `src/memory_budget.rs` with:
  - `pub struct ContextBudget { total_tokens: usize, per_item_chars: usize }`
  - `pub fn enforce(memory: &mut MemoryState, budget: &ContextBudget,
    priority: impl Fn(&WorkingMemoryItem) -> u8)`
  - Token estimate reuses `estimate_tokens_from_chars` from runtime.rs.
- Move `prioritize_evaluative_review_memory` and `prioritize_edit_memory`
  (`src/runtime.rs:782` and `:795`) behind the same `priority` trait so
  ordering and eviction share one source of truth.
- Eviction policy: drop lowest-priority items first; for `Text` items above
  `per_item_chars`, truncate with a `…[truncated N chars]` marker (mirroring
  `trimmed_history`'s pattern); never evict `Current Code Facts` or
  `Likely Edit Targets` in edit mode.
- Compression escape hatch: when even truncation isn't enough, replace the
  item body with `WorkingMemoryItem::Text { label, content: summarize(...) }`
  built from a heuristic (first/last N lines + counts). A model-based summary
  is deferred — heuristic is enough for now.
- New env vars in `src/config.rs`:
  - `RUSTOPEDIA_CONTEXT_TOTAL_TOKENS` (default tied to model profile, see §7)
  - `RUSTOPEDIA_CONTEXT_PER_ITEM_CHARS` (default `4000`, matching smallcode)
- Call site: `synthesize_prompt` (`src/runtime.rs:531`) calls
  `memory_budget::enforce` before `build_prompt`.

### Risks
- `cargo` and `rust-analyzer` summaries are already pre-summarized by the
  tools; double-truncation could erase the one diagnostic line that mattered.
  Mitigation: tool layer should mark its own outputs as "already trimmed"
  and the budget engine should prefer dropping over truncating those items.

## 2. Two-Stage Tool Routing

### Smallcode
> The model first selects a category (read/write/search/run/plan), then
> receives only relevant tool schemas. Critical for models with 8-16k
> context limitations.

### Rustopedia today
Rustopedia doesn't expose tool schemas to the model at all — it does
intent-first routing in `src/intents.rs:81` (`score_intents`) and then
`generate_plan` in `src/planner.rs:10` picks at most one external tool
(`crate_agent`/`docs_agent`/`github_agent`) before the LLM is even called.
This is already a one-stage version of what smallcode does: classify intent,
then only fire the relevant retrieval path.

### Fit
Partial fit. The shape of "classify, then narrow what's loaded" already
applies — what's missing is:
1. The classifier is keyword scoring (deterministic), not LLM-driven. That's
   actually a strength on small models, not a weakness; keep it.
2. The "schemas" being narrowed are mode instructions and tool result
   formatters, not function-call schemas. Today every `ask_instruction`,
   `review_instruction`, and `edit_instruction` variant lives in the same
   match arm and the chosen one is inlined into the prompt — already narrow.

The genuine missing piece: when we eventually add function-calling for the
edit loop (see §3 and §4), the tool-call schemas themselves will need
stage-2 filtering. Until that exists, this highlight is mostly already
satisfied.

### Proposed changes
- Short term: document explicitly in `MODES.md` that `RustIntent` is
  Rustopedia's "stage 1 category" and that prompt content + tool plan
  derived from it is the "stage 2 narrowed payload." No code change.
- Medium term (after §3 patch tools land):
  - `src/tool_routing.rs` exposes `pub fn schemas_for(intent: RustIntent,
    mode: SessionMode) -> Vec<ToolSchema>` so the edit loop sends only the
    relevant subset (e.g., `apply_patch`, `cargo_check`, `read_file` for
    `CompileFix`; not `crate_search`).
  - Keep the keyword classifier; promote it to "tool category gate."

### Risks
- None now. The risk shows up later as duplicated routing logic between
  `generate_plan` and `schemas_for` if both exist; keep them in the same
  module from day one.

## 3. Patch-First Editing

### Smallcode
> Uses search-and-replace as the primary edit strategy rather than full-file
> rewrites. Acknowledges that smaller models "truncate, hallucinate, or
> drift" when trying to reproduce entire files.

### Rustopedia today
This is the most important gap in the entire pipeline. Rustopedia's `edit`
mode does not actually edit files. `execute_turn` ends at
`generate_response` (`src/runtime.rs:213`) and returns a prose answer; the
user copies the suggested change manually. `edit_context.rs` and the
`Likely Edit Targets` working-memory item exist, but only as inputs to the
prompt — there is no write path.

The `edit_instruction` for the catch-all case (`src/generate_prompt.rs:116`)
already nudges the model toward `Current code: / Observed issue: / Minimal
change: / Verification:`. That's a hand-built precursor to a patch format.

### Fit
Highest-leverage change in the list. Turns `edit` from advisor into actor
and is what makes every other smallcode feature (TODO planning, early-stop,
verification, escalation) worth building.

### Proposed changes
- New module `src/patch.rs`:
  ```rust
  pub struct Patch {
      pub path: String,
      pub edits: Vec<PatchEdit>,
  }

  pub enum PatchEdit {
      SearchReplace { search: String, replace: String },
      InsertAfter { anchor: String, content: String },
      CreateFile { content: String },
  }

  pub fn parse_patches(model_output: &str) -> Result<Vec<Patch>, PatchParseError>;
  pub fn apply(patch: &Patch, project_root: &Path) -> Result<PatchOutcome>;
  ```
- Patch format choice: fenced-block search/replace, smallcode-style. Example
  the model must produce:
  ```
  ```patch path=src/config.rs
  <<<SEARCH
  pub model_name: String,
  SEARCH>>>
  <<<REPLACE
  pub model_name: String,
  pub edit_model_name: String,
  REPLACE>>>
  ```
  ```
  Rationale: small models handle exact literal blocks better than unified
  diffs with line numbers. Line numbers drift; anchors don't.
- Update `edit_instruction` (`src/generate_prompt.rs:98`) so every edit
  intent's prompt requires the patch block format and forbids "here is the
  whole file" rewrites. Keep the existing `Current code: / Observed issue:`
  preamble so the model still narrates its reasoning above the patch.
- Add `edit-only` write guard at the runtime layer: after
  `generate_response` returns in edit mode, parse patches, prompt the user
  with a unified-diff preview, apply only on confirmation. (Default to
  preview-then-confirm; add `RUSTOPEDIA_EDIT_AUTOAPPLY=1` for trusted runs.)
- New `WorkingMemoryItem` variant `PatchAttempt` capturing
  `{ patch: Patch, apply_result: ApplyOutcome }` so the next turn (§4) can
  see what just happened.
- Surface failures clearly: if `SEARCH` block doesn't match the file,
  return the failure into working memory and let the model retry — this is
  the loop §4 needs.

### Risks
- The search anchor must be unique in the file; smallcode handles this with
  retry-on-ambiguity. Plan for the same. If a search block matches multiple
  times, fail the patch and surface the error to the model with a directive
  to widen the anchor.
- Whitespace-sensitive matching is brittle. Normalize trailing whitespace
  before compare but never when writing.
- This change requires a `MANUAL_TESTS.md` section: real edits applied to a
  scratch file to validate the parser before pointing it at user code.

## 4. TODO-Driven Planning

### Smallcode
> Complex tasks decompose into atomic steps. The model reads a TODO file
> each turn to know where it is. Each step is validated (lint/compile)
> before moving on.

### Rustopedia today
`PlanStep` (`src/planner.rs:4`) exists but is single-stage — it only
chooses retrieval tools, not multi-turn execution steps. There is no
persistent step state across turns; `SessionState::history` keeps
query/response pairs but no notion of "step 2 of 4."

### Fit
High-value, but strictly dependent on §3. Without patch-first editing,
there is nothing for steps to validate.

### Proposed changes
- New module `src/todo_plan.rs`:
  ```rust
  pub struct TodoPlan {
      pub goal: String,
      pub steps: Vec<TodoStep>,
  }

  pub struct TodoStep {
      pub description: String,
      pub status: StepStatus,
      pub patches: Vec<Patch>,
      pub verification: Vec<VerifyCheck>,
  }

  pub enum StepStatus { Pending, InProgress, Done, Failed(String) }
  pub enum VerifyCheck { CargoCheck, CargoClippy, CargoTest }
  ```
- Persistence: the active plan lives on `SessionState` and is also written
  to `target/rustopedia/active_plan.json` so a crash or `exit` doesn't lose
  state. (Distinct from `.claude` files; this is Rustopedia's own scratch.)
- New session command `/plan` to print the current plan; `/plan reset` to
  clear it. `/status` (`src/runtime.rs:115`) gains a line for active plan.
- Runtime contract: in edit mode, if a plan is active, the next turn's
  prompt starts with the TODO list and the current step description,
  similar to smallcode's "model reads TODO each turn." The model's job is
  to produce patches for the current step only.
- Auto-advance: after `apply` + verification for a step, advance to next
  step or mark `Failed` and surface the failure to the user.
- Plan creation: triggered when (a) user explicitly asks for a multi-file
  feature, or (b) the model emits a `<<<PLAN ... PLAN>>>` block in its
  first edit response. Avoid forcing a plan for one-line clippy fixes.

### Risks
- Plans can rot fast if the model adds steps speculatively. Cap at 6 steps
  and require each step to name a concrete verification check.
- Cross-turn state means a confused user (e.g., switches to `ask`) needs a
  clean exit path. `/plan reset` and auto-clear on `/mode ask` cover this.

## 5. Forgiving Tool Call Parser

### Smallcode
> Handles messy output from small models by parsing tool calls from JSON,
> YAML, XML, Hermes format, or plain text. Auto-repairs common mistakes
> (wrong param names, type mismatches).

### Rustopedia today
There is no tool-call surface — the model writes a single prose response.
Input parsing is limited to slash-commands in `src/session.rs`'s
`parse_input`. The patch parser proposed in §3 will be Rustopedia's first
real "model-output parser" and is the first place forgiving parsing
actually matters.

### Fit
Becomes relevant once §3 and §4 land. Critical then; until then it solves
a problem that doesn't exist.

### Proposed changes (when §3 lands)
- `src/patch.rs::parse_patches` should accept:
  - The canonical fence form (`<<<SEARCH ... SEARCH>>>` etc.)
  - Common drifts: bare ` ```diff ` blocks with `-`/`+` lines, `// SEARCH:`
    inline anchors, raw quoted strings the model labelled "old/new".
  - Path normalization: `./src/foo.rs`, `src/foo.rs`, `/abs/.../src/foo.rs`
    all resolve to the same project-relative path.
- Per-format unit tests under `src/patch/tests.rs` enumerating drifts seen
  from each model in `RUSTOPEDIA_*_MODEL_NAME`.
- For multi-step edit loops, the same module parses `<<<PLAN ... PLAN>>>`
  and `<<<DONE>>>` markers the model may use to advance steps.
- A structured "parse error → repair prompt" path: if parsing fails, the
  next turn's prompt includes the exact malformed block and asks for it
  again in canonical form, instead of just retrying the whole task.

### Risks
- Forgiving parsers are quietly easy to over-fit. Anchor the test suite on
  real model outputs (captured during manual runs) rather than synthetic
  examples.

## 6. Working Memory

### Smallcode
> A persistent scratchpad that survives across turns. Compensates for
> limited reasoning depth — the model can write notes to itself.

### Rustopedia today
`MemoryState` (`src/memory.rs:5`) and `WorkingMemoryItem` (`:20`) already
implement the working-memory concept, but only within a single turn — every
turn rebuilds `working_memory` from scratch in `gather_context`
(`src/runtime.rs:222`). The only cross-turn state is the literal
conversation history (`SessionState::history`) and a small
`MemorySnapshot` displayed by `/memory`.

### Fit
Partial fit. Rustopedia has the data model; it needs the persistence layer.
Smallcode's "scratchpad" maps best to two specific needs Rustopedia has:
- Survival of `Likely Edit Targets` and `Current Code Facts` between turns
  in the same edit task, so a follow-up doesn't repay the analysis cost.
- A place for the model itself to write notes (e.g., "investigated
  `LlmClient::generate_openrouter`, no issue found") so it doesn't
  re-explore the same files turn after turn.

### Proposed changes
- New field on `SessionState`:
  ```rust
  pub struct ScratchPad {
      pub notes: Vec<ScratchNote>,         // model-authored
      pub pinned: Vec<WorkingMemoryItem>,  // runtime-pinned, e.g. edit targets
  }
  ```
- `WorkingMemoryItem` variant `ScratchNote { from_turn: usize, text: String }`
  rendered into prompts under a `### Scratchpad:` section between
  `Session Summary` and `Working Memory`.
- Model contract: the `edit_instruction` prompts (`src/generate_prompt.rs:99`)
  gain an optional `<<<NOTE ... NOTE>>>` block whose contents the parser
  routes into `ScratchPad::notes`.
- Pinning rules: when `gather_edit_context` returns targets, those are
  pinned for the next 3 turns or until `/mode` changes. Lets a multi-turn
  edit conversation keep its anchor without re-running grep.
- Auto-eviction: scratch entries older than 20 turns or 6_000 chars are
  dropped by the context budget engine from §1.

### Risks
- The model will sometimes write garbage notes. Cap each note at ~400 chars
  and require a leading verb (`Investigated`, `Confirmed`, `Ruled out`) to
  keep notes terse. Reject anything that looks like a reasoning dump.

## 7. Model Profiles

### Smallcode
> Per-model configuration including context length, tool format, chat
> template, and strengths/weaknesses for adaptive prompting.

### Rustopedia today
`AppConfig` (`src/config.rs:31`) has `model_name`, `review_model_name`, and
`edit_model_name` as opaque strings. `LlmClient::generate`
(`src/llm.rs:23`) branches on provider, not on model — so a 3B
quantization is treated the same as a 70B remote model in terms of prompt
shape, max context, and what to ask it to do.

### Fit
Strong fit. Every other feature in this doc (context budget, patch format
strictness, escalation thresholds) wants this struct as input.

### Proposed changes
- New module `src/model_profile.rs`:
  ```rust
  pub struct ModelProfile {
      pub id: String,                  // "deepseek-coder-v2:latest"
      pub provider: LlmProvider,
      pub context_window_tokens: usize,
      pub good_at_patches: bool,
      pub good_at_long_context: bool,
      pub prefers_strict_format: bool, // tighten patch fences if true
      pub escalation_target: Option<String>, // e.g. "anthropic/claude-..."
  }

  pub fn for_id(id: &str) -> ModelProfile;
  ```
- Built-in profiles for the models Rustopedia commonly sees
  (`deepseek-coder-v2`, `qwen2.5-coder`, `llama3.1`, common OpenRouter
  models). Unknown models get a conservative default (8k context, generic
  prompt style).
- Override path: `RUSTOPEDIA_MODEL_PROFILE_FILE` points to a TOML or JSON
  file letting users add profiles without recompiling.
- Wire-up:
  - `ContextBudget` reads `context_window_tokens` per call.
  - `build_prompt` selects a stricter patch-format example when
    `prefers_strict_format = true`.
  - `escalate_if_needed` (§9) reads `escalation_target`.
- `/status` prints the active profile so users can confirm.

### Risks
- Profiles drift as model releases land. Treat the built-in profile table
  as defaults, not gospel; ship the override file path from day one.

## 8. Early-Stop Detection

### Smallcode
> Detects repetition loops, patch spirals (stuck on corrupted file → forces
> rewrite), and greeting regression (model lost context → re-injects task).

### Rustopedia today
Single-shot turns can't loop, so there's nothing to detect. The closest
thing is the post-analysis clarification gate in
`post_analysis_clarification` (`src/runtime.rs:745`) and the contextual
follow-up shortcut (`:1024`).

### Fit
Becomes critical the moment §4's TODO loop lands. Until then, defer.

### Proposed changes (when §4 lands)
- New module `src/loop_guard.rs` invoked at the end of each step in the
  TODO executor:
  ```rust
  pub enum LoopSignal {
      Continue,
      Stop { reason: String },
      ForceRewrite { path: String },
      ReinjectTask,
  }

  pub fn check(history: &PlanExecutionLog, plan: &TodoPlan) -> LoopSignal;
  ```
- Heuristics, all cheap:
  - **Repetition:** same patch text attempted twice — `Stop`.
  - **Patch spiral:** same file fails to apply 3+ turns in a row —
    `ForceRewrite` (model is asked to write a full file replacement next
    turn, scoped to that one file only, since search/replace is failing).
  - **Greeting regression:** model output starts with "Hello!" / "Sure, I
    can help" instead of a patch or verification — `ReinjectTask` re-pastes
    the goal + current step at the top of the next prompt.
  - **No-op detection:** verification result identical 3 turns in a row →
    `Stop` and surface "no progress" to the user.
- Hard cap: max 8 turns per plan or 6 turns per step, whichever first.
  Configurable via `RUSTOPEDIA_EDIT_MAX_TURNS` / `_PER_STEP`.

### Risks
- Aggressive `ForceRewrite` can blow context budget on large files. Gate it
  on file size and fall back to "stop and ask user" if the file is large.

## 9. Model Escalation

### Smallcode
> Optional fallback to stronger cloud models (Claude, OpenAI, DeepSeek)
> only on hard failures. Requires API keys; fully opt-in to prevent
> runaway costs.

### Rustopedia today
The plumbing is already 80% there: `LlmClient` (`src/llm.rs:7`) speaks
both Ollama and OpenRouter, and `AppConfig` holds both base URLs plus the
OpenRouter API key. What's missing is the policy: "when do we escalate?"
and "to what?"

### Fit
Clean fit. Most of the work is policy + a second `LlmClient` construction;
no new infrastructure required once §7 lands (escalation target lives on
the model profile).

### Proposed changes
- Extend `LlmClient` to hold a primary and an optional escalation
  configuration, or — cleaner — keep two `LlmClient` instances on
  `Runtime`.
- New module `src/escalation.rs`:
  ```rust
  pub enum EscalateTrigger {
      PatchParseFailedRepeatedly(usize),
      VerificationFailedRepeatedly(usize),
      ContextBudgetExceeded,
      ExplicitUserRequest,
  }

  pub fn should_escalate(trigger: &EscalateTrigger, profile: &ModelProfile) -> bool;
  ```
- Trigger thresholds default to 2 (i.e., escalate on the third failure of
  the same kind in one turn). Cost guard: never escalate more than once
  per user turn unless `/escalate` is typed.
- New session command `/escalate` to force the next turn to use the
  escalation model, regardless of trigger state.
- Opt-in by configuration: escalation is disabled unless the active
  `ModelProfile` has `escalation_target = Some(...)` and the right env
  var (`RUSTOPEDIA_OPENROUTER_API_KEY` or future provider keys) is set.
- Surface visibly: print `Escalating to <model>` before the call. No
  silent fallbacks.

### Risks
- Cost. Defaults must be conservative; the opt-in chain should be explicit
  (profile gate + key present + threshold met). `/status` should print
  "escalation: enabled / disabled" so users know.

## Phasing

These phases are ordered by dependency, not by user-visible value. Phase 2
is what makes Rustopedia genuinely different from today.

### Phase 1 — Foundations (low risk, no behavior change)
- §7 Model Profiles
- §1 Context Budget Engine (uses profile context window)
- §6 Working Memory cross-turn pinning (no model contract change yet, just
  re-injecting `Likely Edit Targets` on the next turn)

### Phase 2 — Make edit mode actually edit
- §3 Patch-First Editing (preview-then-confirm by default)
- §5 Forgiving parser (the patch-parser flavor of it)
- Update `MODES.md` to reflect the new edit contract.

### Phase 3 — Multi-step edits
- §4 TODO-Driven Planning
- §8 Early-Stop Detection
- §6 Working Memory scratch-notes from the model
- `/plan`, `/plan reset` commands

### Phase 4 — Reach outside when needed
- §9 Model Escalation
- §2 stage-2 tool schema filtering (only meaningful once §3 brings
  function-callable tools into the loop)

## Open decisions to lock down before Phase 1

These are the questions whose answers shape several modules at once and are
cheap to resolve now and expensive to change later.

1. **Patch format syntax.** `<<<SEARCH ... SEARCH>>>` vs unified diff vs
   fenced ` ```patch ` blocks. The doc proposes search/replace; confirm
   before §3 ships.
2. **Plan persistence path.** `target/rustopedia/active_plan.json` is the
   proposed default. Alternative: `.rustopedia/` at the project root, more
   visible but pollutes git status.
3. **Escalation default off vs default on when a key is present.** Doc
   proposes "off unless the profile opts in"; smallcode is "off unless
   triggered." Pick one.
4. **Scratchpad note authorship.** Model-authored only, or runtime can
   also pin notes (e.g., "last cargo check: clean")? Doc proposes both,
   with a `from_turn`/`source` tag.
5. **What to do with the legacy `rag/` Python pipeline once §1 and §7 land
   for Ollama/OpenRouter.** Not blocking, but the model-profile work is a
   natural moment to revisit whether Qdrant retrieval should remain
   first-class or move behind a feature flag.
