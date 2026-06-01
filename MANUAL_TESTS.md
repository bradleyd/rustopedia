# Manual Test Checklist

Use this file as a lightweight product-level test pass while Rustopedia is still evolving.

For each prompt, record:

- mode
- prompt
- expected behavior
- actual route/tools
- latency impression
- answer quality notes
- regressions or surprising behavior

## Ask Mode

### Rust language and stdlib

- `what does Pin<&mut T> guarantee?`
- `when should I use Rc vs Arc?`
- `how do I use OnceLock?`
- `what does tokio::select! do?`

Expected:

- direct Rust answer
- no unnecessary local workspace analysis
- no generic filler

### Local workspace explanation

- `how does this project load configuration?`
- `what does this runtime do?`
- `where is session state stored?`
- `how does ask mode route tools?`

Expected:

- grounded in local code
- cites the real local behavior, not generic Rust examples
- no invented code snippets when local evidence is available

Watch for:

- generic template answers
- claims that are not supported by local files
- failure to stay anchored on repo details

### Crate selection

- `what crate should I use for retries?`
- follow with `http`
- then `can you show me an example?`
- `what crate should I use for config loading?`
- `what crate should I use for parsing CLI args?`

Expected:

- broad asks clarify first when needed
- follow-up keeps subject and use case
- examples stay about the selected subject
- no unrelated crate recommendations

Watch for:

- subject loss on short follow-ups
- noisy or niche crates presented as obvious defaults
- examples that are generic and not tied to the prior crate discussion

### Ambiguous asks

- `can you help?`
- `what should I use here?`
- `why?`
- `which one?`
- `show me an example`

Expected:

- uses mode plus recent subject when available
- does not overcommit too early
- asks for clarification or reuses prior subject appropriately

## Review Mode

### Project explanation

- `what does this project do?`
- `what problem is this project solving?`

Expected:

- concise explanation first
- grounded in local code and runtime behavior
- no generic repo-summary boilerplate

### Architecture and code paths

- `walk me through the main architecture`
- `trace the ask-mode request flow`
- `how does the configuration path work?`

Expected:

- real modules and responsibilities
- codepath-level explanation
- avoids generic architecture-speak

### Evaluative review

- `what is wrong with this project?`
- `where are the brittle spots?`
- `what are the biggest maintenance risks?`

Expected:

- findings first
- evidence-backed
- uses compiler/lint/test/local-code evidence where relevant

Watch for:

- generic review filler
- security/testing boilerplate not tied to evidence

## Edit Mode (Patching Dry-Run)

These exercises validate the SEARCH/REPLACE patch parser and the
`PATCH OUTPUT REQUIREMENT` prompt block. The runtime is in dry-run mode:
file writes are not yet wired up. Every edit-mode turn appends a
`--- Planned Patches (dry-run, no files written) ---` section to the model
response (or a `--- Patch Parse Errors ---` section when parsing fails).

For each prompt, record on top of the usual fields:

- **format:** did the model emit `<<<SEARCH ... SEARCH>>>` / `<<<REPLACE ... REPLACE>>>` blocks at all?
- **parse:** did `patch::parse` accept the blocks? If errors, paste the error line.
- **anchor:** does the SEARCH text actually exist in the named file byte-for-byte, and is it unique?
- **path:** is `path=` a real project-relative path?
- **semantic:** if the patch *were* applied, would it be correct? (Yes / partially / no)
- **drift:** any unified diff, line numbers, full-file rewrites, or other format violations.

### Single-line existing-file edits

Drive these from a clean tree so the dry-run preview is the only diff source.

- `add a #[derive(Default)] above the ScratchPad struct in src/session.rs`
  - Note: ScratchPad does not exist yet; the model should refuse rather than fabricate one. Tests the "no evidence → emit no patch" rule.
- `change the default ollama base url to http://127.0.0.1:11434 in src/config.rs`
- `add a TODO comment above the model_for_mode function in src/config.rs`
- `rename the local variable q to query inside infer_initial_intent in src/intents.rs`

Expected:

- exactly one patch block per change
- SEARCH text matches the file byte-for-byte (verify by grep)
- prose explanation above the block, no prose between SEARCH and REPLACE markers
- no `@@`, no leading `+`/`-` lines outside the block

Watch for:

- model proposes the change in prose only, no patch block (prompt drift)
- SEARCH text approximates the file instead of matching it
- model picks too-narrow an anchor (e.g., a bare `}`) that would match many places

### Multi-line and multi-edit cases

- `add two new fields scratch_notes: Vec<String> and pinned_edits: Vec<String> to AppConfig in src/config.rs, then default both to empty in from_env`
- `add a new variant TestCreationFollowUp to RustIntent in src/intents.rs and handle it in as_str`
- `wrap the cargo_check_summary call in a tracing::info! span in src/runtime.rs`

Expected:

- one patch block per file, multiple SEARCH/REPLACE pairs inside when the same file is touched in two places
- consistent path attribute
- preview shows N edits clearly numbered

Watch for:

- model splits one file's changes across multiple patch blocks (acceptable but noisy)
- second edit's SEARCH overlaps the first edit's REPLACE (would fail on apply once writes land)

### New-file creation

- `create a new module src/scratchpad.rs with a stub ScratchPad struct and a new function`
- `create a new file src/tools/notes.rs that re-exports the scratchpad types`

Expected:

- patch header reads `path=src/scratchpad.rs new=true`
- body is the file contents, no SEARCH/REPLACE markers
- preview line reads `[1] create src/scratchpad.rs (N line(s))`

Watch for:

- model uses SEARCH/REPLACE form on a file that does not exist yet (parse error: SEARCH won't match)
- model omits `new=true` and tries to "edit" the empty file
- file content extends beyond the closing fence (parse fails)

### Refusal under thin evidence

- `fix the failing test in src/runtime.rs` (no failing test exists)
- `repair the borrow checker error in main.rs` (no error exists)
- `make the patch parser handle YAML-style edits` (no concrete target, ambiguous scope)

Expected:

- the prompt's `PATCH OUTPUT REQUIREMENT` rule "say so explicitly and emit no patch rather than guessing" should take effect
- response should explain insufficient evidence and emit zero patch blocks
- `--- Planned Patches ---` section should not appear when no blocks are emitted

Watch for:

- model fabricates a SEARCH block with code that doesn't exist
- model emits a partial patch and apologizes — count as a regression and tighten the prompt

### Parser stress (intentional malformed output)

These verify the parse-error surface is useful, not that the model produces
them. Easiest way to drive them is to ask for something that historically
produces drift, then read the error preview.

- `fix this with a unified diff` — should produce a unified diff, the parser should reject the patch block entirely (no `<<<SEARCH` marker), and the error should mention the missing marker.
- `rewrite src/config.rs in full to remove the deprecated fields` — model may emit a whole-file block without `new=true`; expected error mentions "no SEARCH/REPLACE pairs and is not marked new=true".

Expected:

- `--- Patch Parse Errors ---` section appears with a single concrete error line
- error excerpt shows the first ~80 chars of the offending block
- runtime continues normally on the next turn

### Long-session behavior with patches

Run 5 edit-mode prompts in a row, mixing single-edit and refusal cases.

Expected:

- parser does not get confused by stale patch blocks in prior turn history
- `Current Code Facts` in working memory continues to suppress invented structs across turns
- no growth in parse-error rate as the session lengthens

Watch for:

- model reuses an old patch verbatim from history instead of writing a new one for the current ask
- prompt size growth pushing the patch requirement out of context (would only happen on small `num_ctx` models — check `/status`)

## Session Behavior

### Clarification and follow-up carryover

- ask a broad crate question
- answer with a one-word clarification
- ask `can you show me an example?`
- ask `explain more`
- ask `why?`

Expected:

- one-word clarification should continue prior subject
- short follow-ups should stay anchored to the last subject
- no reset to generic Rust Q&A

### Command handling

- `/mmode ask`
- `/statuz`
- `/help`
- `/status`

Expected:

- unknown slash commands fail fast
- valid commands do not hit the model

### Long-session drift

Run 6-10 prompts in one session across `ask` and `review`.

Expected:

- later turns do not balloon in prompt size unnecessarily
- recent subject is preserved when useful
- old unrelated turns do not dominate answers

Watch for:

- rising latency
- stale context reuse
- answers drifting into previous topics

## Failure Modes

- ask a non-Rust question such as `what restaurant should I go to?`
- ask a local repo question when tooling is unavailable
- ask for an external example when GitHub lookup fails

Expected:

- Rustopedia stays within product scope
- failures are clear and narrow
- it does not pretend unsupported capabilities worked

## Known Weak Spots To Recheck

- local workspace asks that currently drift into generic Rust examples
- crate-selection quality for broad ecosystem questions
- example correctness when the model answers from subject carryover instead of external grounding
- long-session prompt growth on local models
