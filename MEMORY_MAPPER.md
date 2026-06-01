# Memory Mapper Design

This document defines the next memory architecture for Rustopedia.

The goal is not "chat history management" in the generic assistant sense.

The goal is:

- keep local source-of-truth fresh
- keep prompt size bounded for local models
- avoid answering from stale summaries when real file evidence is available
- support future edit workflows without stuffing whole files into the prompt

## Problem

The current runtime still treats prompt context too much like a flat string:

- recent conversation is appended into `Past Conversation`
- local context chunks are joined together
- broad summaries and targeted evidence can coexist without clear priority

This causes several problems:

- local models get slower over long sessions
- broad summaries can drown out precise file evidence
- stale conversation can steer the answer more than fresh code
- specific file questions can still get generic answers

## Design Principles

1. Fresh source-of-truth beats old summary.
2. Current-task evidence should be small and high-trust.
3. Older context should be compressed aggressively.
4. Broad repo summaries and targeted code excerpts should live in different layers.
5. File mentions should trigger fresh file-backed reads, not summary-only reasoning.

## Separate Two Concepts

Rustopedia should not treat these as the same thing:

1. Evidence retrieval
2. Prompt memory

### Evidence retrieval

This is how Rustopedia reads fresh truth from the workspace or tools:

- file excerpts
- rust-analyzer summaries
- cargo diagnostics
- crate/docs/example lookup

This data is ephemeral and should be refreshed when needed.

### Prompt memory

This is how Rustopedia carries forward previous session state:

- previous clarifications
- selected subject
- prior decisions
- compressed conversation summaries

This data is persistent within the session, but lower trust than fresh evidence.

## Memory Layers

The prompt should be assembled from explicit layers.

### L0: Current Task

Small, explicit task frame:

- current mode
- current inferred intent
- current user query
- any active clarification state
- whether the current turn is a contextual follow-up

This should always be present.

### L1: Working Memory

Fresh, high-priority evidence for the current turn.

Examples:

- file excerpts with line numbers
- targeted local code context
- current compiler/lint/test diagnostics
- current external lookup results
- current selected crate/doc/example references

Properties:

- highest trust
- short-lived
- replaced every turn
- should be as small as possible

### L2: Session Working Summary

Compressed session state that is still likely relevant.

Examples:

- the user clarified that retries means HTTP
- the last selected subject was `reqwest-retry`
- the current task is focused on `src/config.rs`
- a previous turn established that the project loads config via `AppConfig::from_env()`

Properties:

- medium trust
- rewritten periodically
- should be compact prose or structured bullets
- should not contain large raw excerpts

### L3: Background Summary

Older, lower-priority context retained in compact form.

Examples:

- high-level repo summary
- prior architectural conclusions
- older resolved discussions

Properties:

- lowest trust
- only included when useful
- should be easy to drop when the prompt budget is tight

## Core Data Structures

These do not need to be fully implemented at once, but they should define the target shape.

```rust
struct MemoryState {
    working_memory: Vec<WorkingMemoryItem>,
    session_summary: SessionSummary,
    background_summary: Option<BackgroundSummary>,
    anchors: MemoryAnchors,
}

struct MemoryAnchors {
    current_subject: Option<SubjectAnchor>,
    pending_clarification: Option<PendingClarification>,
    last_verification: Option<VerificationSnapshot>,
}

enum WorkingMemoryItem {
    FileExcerpt(FileExcerpt),
    ToolOutput(ToolOutputSummary),
    Diagnostic(DiagnosticExcerpt),
    LookupResult(LookupSummary),
}

struct FileExcerpt {
    path: String,
    start_line: usize,
    end_line: usize,
    text: String,
    freshness_epoch: u64,
}

struct SessionSummary {
    text: String,
    updated_turn: usize,
}

struct BackgroundSummary {
    text: String,
    updated_turn: usize,
}
```

## File Reading Strategy

Rustopedia should use normal Rust file I/O as the baseline source-of-truth path.

Recommended implementation direction:

- `std::fs::read_to_string`
- split into lines
- extract line-numbered ranges

This is enough to support:

- read named file
- read excerpt
- find matching lines
- refresh context after edits

It is a better foundation than keeping raw file content in long-lived prompt history.

### File-related tools to add

- `read_file(path)`
- `read_file_excerpt(path, start_line, end_line)`
- `find_in_file(path, query_terms)`
- `resolve_named_file(user_query)`
- `summarize_file(path)` as a fallback, not a primary source of truth

## Update Rules

### On every turn

1. Clear previous working memory.
2. Rebuild working memory from fresh evidence for the current task.
3. Keep session summary and background summary unless intentionally rewritten.

### When the user names a file

1. Resolve the file path.
2. Read the file or relevant excerpts fresh.
3. Put the excerpts in working memory.
4. Downweight broad repo summary for that turn.

### When the user asks a broad repo question

1. Use repo-level summaries such as `project_overview`.
2. Only read targeted files if the summary is insufficient.

### When the user asks a short follow-up

1. Use the last subject anchor.
2. Decide whether to refresh working memory from the anchored files or lookup subject.
3. Avoid re-running broad retrieval unless needed.

### After edits

1. Invalidate any working-memory file excerpts that touched changed files.
2. Refresh from disk.
3. Refresh diagnostics.
4. Update session summary with the new state.

## Prompt Assembly

The prompt should stop being one flat combination of:

- full past conversation
- all context chunks

Instead it should become something like:

1. current mode and intent
2. current task
3. working memory
4. session summary
5. optional background summary

Suggested prompt sections:

- `Current Task`
- `Working Memory`
- `Session Summary`
- `Background Summary`
- `Answer`

## Compression Rules

### Compress into session summary when

- working memory from prior turns is still relevant conceptually
- raw excerpts are no longer needed
- repeated clarifications accumulate

### Compress into background summary when

- session summary grows too large
- older decisions are still worth remembering in compact form
- the current task has shifted substantially

### Drop entirely when

- a prior topic is unrelated to the current subject
- fresh file evidence supersedes old summary
- prompt budget becomes tight

## Freshness Policy

Every file-backed working-memory item should be treated as stale after:

- the file is edited
- a new turn explicitly re-targets the same file
- the mode changes into an edit-oriented workflow

Session summaries can remain valid longer, but they must always lose to fresh reads.

## Budgeting

Local models require explicit prompt budgets.

Suggested policy:

- reserve most of the budget for working memory
- keep session summary small and dense
- include background summary only when the current turn truly benefits from it

Example priority when trimming:

1. current task
2. file excerpts / diagnostics / current tool results
3. short session summary
4. background summary
5. old conversation text

## Implications For Edit Mode

This memory mapper is a prerequisite for strong edit workflows.

Why:

- edit mode needs fresh file truth after every patch
- old file content must not linger as if it were current
- touched files should stay in working memory
- prior reasoning should collapse into session summaries, not remain as raw prompt text

## Immediate Next Steps

1. Add explicit `WorkingMemoryItem` and `FileExcerpt` types.
2. Introduce file-read tools that return line-numbered excerpts.
3. Change prompt assembly to use `working_memory + session_summary + background_summary`.
4. Route explicit file mentions through fresh file reads before generation.
5. Add invalidation rules so edited files force fresh reads.

## Non-Goals For The First Version

- vector memory as the main local-code truth source
- whole-file persistence in prompt history
- fully autonomous summarization loops after every turn
- perfect symbolic indexing before the file-read path exists

The first version should be simple, explicit, and grounded:

- fresh file reads
- small working memory
- compact session summary
- optional background summary
