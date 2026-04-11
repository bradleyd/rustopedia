# Rustopedia Design

## Goal

Rustopedia is a local Rust-focused coding assistant.

It should do five things well:

1. Explain a Rust codebase.
2. Answer Rust-specific questions.
3. Fix and refactor the current codebase.
4. Create tests for the current codebase.
5. Create new Rust code that fits the local project.

The design target is "Codex or Claude for Rust work, but local-first and grounded in the current repository."

## Product Scope

Rustopedia is not a general-purpose assistant.

It is optimized for:

- local repository understanding
- Rust language and standard library questions
- Rust crate usage questions
- code review and code explanation
- code edits with verification
- test generation

It is not optimized for:

- broad non-Rust research
- unverified autonomous edits
- long-running fully autonomous project rewrites
- internet-first answers when the local repository already contains the evidence

## User-Facing Modes

Rustopedia has three explicit modes.

### `ask`

Use for:

- Rust language questions
- standard library questions
- crate API questions
- snippet explanation

Behavior:

- no file edits
- answer directly
- use local repo context when relevant
- use external Rust sources only when needed

### `review`

Use for:

- explaining code in the current repo
- tracing behavior or architecture
- reviewing code quality or correctness
- identifying bugs, risks, and regressions

Behavior:

- no file edits by default
- if the request is explanatory, provide a walkthrough
- if the request is evaluative, provide findings first
- ground the answer in local repo evidence first

### `edit`

Use for:

- fixing code
- refactoring code
- creating code
- creating tests
- repairing tests or clippy issues

Behavior:

- file edits are allowed
- touched files should be reported
- verification should run when possible
- failures should drive a retry loop when practical

## Mode Switching

Mode is explicit and sticky across turns.

Commands:

- `/mode ask`
- `/mode review`
- `/mode edit`
- `/status`
- `/help`

Guardrail:

- only `edit` mode may write files

## Primary Use Cases

### 1. Explain a codebase

Examples:

- "What does this crate do?"
- "Trace this request path."
- "Where is this trait implemented?"
- "What does this code snippet do?"

Primary evidence:

- rust-analyzer symbol and type information
- repo lexical search
- structural search
- generated repo summaries/wiki

### 2. Answer Rust questions

Examples:

- "How do I get the minimum float?"
- "How do I implement `FromStr`?"
- "What is the difference between `Rc` and `Arc`?"

Primary evidence:

- model knowledge
- Rust std docs
- docs.rs when crate-specific
- local repo usage examples when relevant

### 3. Fix the current codebase

Examples:

- "Fix this compile error."
- "Make this pass clippy."
- "Repair the failing tests."

Primary evidence:

- rust-analyzer diagnostics
- `cargo check`
- `cargo test`
- `cargo clippy`
- local code context

### 4. Create tests

Examples:

- "Add unit tests for this parser."
- "Write regression tests for this bug."
- "Improve coverage around this API."

Primary evidence:

- existing test patterns in the repo
- public API shape
- previous bug reports or diagnostics
- code path and edge-case analysis

### 5. Create code

Examples:

- "Add a helper for this type."
- "Implement this feature."
- "Refactor this module."

Primary evidence:

- existing local abstractions
- rust-analyzer context
- compiler feedback
- test/style patterns in the repo

## Retrieval Strategy

Rustopedia should be local-first.

Retrieval order:

1. local code intelligence
2. lexical search
3. structural search
4. compiler and test diagnostics
5. local wiki/summaries
6. external Rust sources
7. optional semantic retrieval

### Why local-first matters

For Rust coding, exact structure is more valuable than semantic similarity.

The agent needs:

- symbol names
- definitions
- references
- type context
- diagnostics
- exact local patterns

This is more important than generic vector search.

## Architecture

Rustopedia should move toward an agent workflow runtime, using `agent-line` as the execution model.

High-level stages:

1. route
2. gather
3. act
4. verify
5. summarize

### Route

Input:

- user message
- current mode
- session state

Output:

- task classification
- tools required
- whether edits are allowed
- whether verification is required

### Gather

Gather should pull the minimum useful evidence before generation.

Sources:

- rust-analyzer
- repo search
- structural search
- diagnostics
- docs.rs / Rust docs
- crates.io / GitHub when needed
- local wiki summaries

### Act

In `ask` and `review`:

- synthesize response only

In `edit`:

- generate a patch plan
- edit files
- capture touched files

### Verify

Verification is required for `edit` when practical.

Checks:

- `cargo check`
- `cargo test`
- `cargo clippy`
- rust-analyzer diagnostics refresh

### Summarize

Return:

- answer or findings
- touched files, if any
- verification results
- any remaining blockers

## Core Backends

### 1. `rust-analyzer`

This should be the primary code intelligence backend.

Use it for:

- document symbols
- workspace symbols
- go-to-definition
- references
- hover/type info
- diagnostics
- possibly code actions later

This is the most important addition for local Rust quality.

### 2. Lexical search

Add a local full-text index over:

- `.rs` files
- README/docs
- test files
- diagnostics snapshots
- generated wiki pages

Preferred direction:

- `tantivy`

This is better than semantic retrieval for exact code questions.

### 3. Structural search

Add AST-aware or syntax-aware search for patterns such as:

- `unwrap` / `expect`
- `unsafe`
- trait impls
- async boundaries
- error propagation
- macros

Preferred direction:

- `ast-grep` or tree-sitter-based search

### 4. Compiler and test feedback

Treat verification output as first-class context.

Store and reuse:

- compile errors
- clippy warnings
- test failures

These are often the highest-value signals for fixes.

### 5. External Rust sources

Keep the existing external retrieval tools, but demote them.

Use them only when local evidence is not enough:

- Rust docs
- docs.rs
- crates.io
- GitHub examples

### 6. Semantic retrieval

Keep Qdrant optional.

Use it for:

- long-form docs
- design notes
- wiki pages
- fuzzy concept lookup

Do not make it the center of the code-editing pipeline.

## Knowledge Layers

### L1: Always-loaded operational context

Small, stable, and always available:

- mode semantics
- edit guardrails
- project coding rules
- verification policy
- response format policy

### L2: Queried local memory

Persistent but loaded on demand:

- repo wiki pages
- module summaries
- trait/impl summaries
- unsafe audit notes
- prior findings
- design notes

The wiki is a synthesized memory layer, not the primary source of truth.

## Runtime Design

`agent-line` should become the orchestration substrate.

Why:

- explicit step graph
- retry and next-step control
- shared context
- tracing hooks
- a clean path to separate `ask`, `review`, and `edit`

### Suggested workflow families

#### Ask workflow

1. classify request
2. gather local and external context
3. answer

#### Review workflow

1. classify review intent
2. gather local context
3. gather diagnostics if needed
4. synthesize walkthrough or findings

#### Edit workflow

1. classify edit request
2. gather local context
3. generate plan
4. edit files
5. verify
6. retry or finish

## Proposed Module Layout

Target structure:

- `src/main.rs`
- `src/runtime.rs`
- `src/session.rs`
- `src/modes/ask.rs`
- `src/modes/review.rs`
- `src/modes/edit.rs`
- `src/workflows/ask.rs`
- `src/workflows/review.rs`
- `src/workflows/edit.rs`
- `src/agents/router.rs`
- `src/agents/rust_analyzer.rs`
- `src/agents/repo_search.rs`
- `src/agents/structural_search.rs`
- `src/agents/docs.rs`
- `src/agents/crates.rs`
- `src/agents/github.rs`
- `src/agents/qdrant.rs`
- `src/agents/cargo_check.rs`
- `src/agents/cargo_test.rs`
- `src/agents/clippy.rs`
- `src/agents/patch.rs`
- `src/agents/finalize.rs`
- `src/wiki/ingest.rs`
- `src/wiki/query.rs`

## Immediate Roadmap

### Phase 1: Runtime alignment

- keep explicit modes: `ask`, `review`, `edit`
- move runtime flow out of `main.rs`
- keep current ask behavior working
- add edit-only write guard

### Phase 2: Local Rust intelligence

- integrate rust-analyzer
- add repo lexical search
- add structural search
- expose diagnostics as tools

### Phase 3: Edit loop

- add patch planning
- add file mutation flow in `edit`
- add `cargo check`, `cargo test`, `cargo clippy`
- add retry loop based on failures

### Phase 4: Test generation

- inspect local test conventions
- generate unit and integration tests
- verify immediately

### Phase 5: Local wiki

- generate repo summaries
- persist L2 notes
- expose summaries to `review` and `edit`

## Success Criteria

Rustopedia is successful when it can reliably:

1. explain unfamiliar Rust code in the current repo
2. answer everyday Rust language and API questions
3. fix common Rust compile and clippy issues
4. add tests that match local repo style
5. create small-to-medium Rust changes and verify them

## Evaluation

Build an eval set around real Rust tasks:

- compile error repair
- lifetime error repair
- trait bound fixes
- API refactors
- clippy cleanup
- regression test creation
- module explanation
- snippet explanation
- crate recommendation with justification

Metrics:

- compile success
- test success
- clippy cleanliness
- review finding quality
- explanation usefulness
- edit latency

## Current Design Decisions

1. Keep the product name `rustopedia` for now.
2. Use explicit sticky modes.
3. Merge "inspect" and "critique" into `review`.
4. Keep Qdrant optional, not central.
5. Make rust-analyzer the primary local intelligence layer.
6. Use local repo evidence before internet retrieval.
