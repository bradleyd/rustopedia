use crate::intents::RustIntent;
use crate::memory::MemoryState;
use serde_json::Value;

use crate::planner::PlanStep;
use crate::session::SessionMode;

pub fn build_prompt_with_retry(
    memory: &MemoryState,
    plan: &[PlanStep],
    retry_directive: Option<&str>,
) -> String {
    let mode = memory.current_task.mode;
    let intent = memory.current_task.intent;
    let query = memory.current_task.query.as_str();
    let mode_instruction: String = match mode {
        SessionMode::Ask => ask_instruction(intent).to_string(),
        SessionMode::Review => review_instruction(intent).to_string(),
        SessionMode::Edit => {
            format!("{}\n\n{}", edit_instruction(intent), PATCH_OUTPUT_REQUIREMENT)
        }
    };

    let workflow_summary = if plan.is_empty() {
        "No external Rust reference tools were selected.".to_string()
    } else {
        plan.iter()
            .map(|step| format!("- {}: {}", step.tool, step.input))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let retry_section = retry_directive
        .filter(|d| !d.trim().is_empty())
        .map(|d| format!("\n\n### Retry Directive (previous attempt had unmatched SEARCH anchors):\n{}", d))
        .unwrap_or_default();

    format!(
        "{mode_instruction}{retry_section}

### Current Mode:
{}

### Rust Workflow:
{}

### Session Summary:
{}

### Working Memory:
{}

### Background Summary:
{}

### User Question:
{}

### Answer:",
        mode.as_str(),
        workflow_summary,
        memory.session_summary.trim(),
        memory.working_memory_text().trim(),
        memory.background_summary_text().trim(),
        query.trim()
    )
}

fn ask_instruction(intent: RustIntent) -> &'static str {
    match intent {
        RustIntent::CrateSelection => {
            "You are Rustopedia, a Rust-first programming pair. Help choose Rust crates pragmatically. Use only the evidence provided in the prompt plus stable Rust ecosystem knowledge; do not invent maintenance dates, feature support, or popularity claims that are not supported by the supplied crate data. For broad queries, prefer a short recommendation with 2-3 candidates max, explain when the choice depends on async runtime or use case, and say explicitly when the search results look noisy or niche instead of pretending there is a definitive winner."
        }
        RustIntent::ApiLookup => {
            "You are Rustopedia, a Rust-first programming pair. Answer the Rust API question directly first, then explain the relevant trait, function, module, or type behavior with precise Rust terminology. Prefer code-level correctness over broad tutorial prose."
        }
        RustIntent::ExampleLookup => {
            "You are Rustopedia, a Rust-first programming pair. Give a compact Rust example first, then explain the important pieces briefly. Keep the answer implementation-focused and avoid generic filler."
        }
        RustIntent::LocalWorkspaceQuestion => {
            "You are Rustopedia, a Rust-first programming pair. Explain the current Rust workspace directly and stay grounded in the supplied local code evidence. When file excerpts are provided, cite the file path and line range in your explanation. If the provided context is insufficient, say that plainly instead of inventing generic Rust examples or placeholder code."
        }
        _ => {
            "You are Rustopedia, a Rust-first programming pair. Stay tightly focused on Rust development work. Prefer local workspace evidence when available. Use crates/docs/examples context only when it directly helps answer the Rust question. Do not drift into generic non-Rust assistant behavior."
        }
    }
}

fn review_instruction(intent: RustIntent) -> &'static str {
    match intent {
        RustIntent::ProjectExplanation => {
            "You are explaining a local Rust project. Answer directly in 1-2 sentences first. Focus on the project’s Rust purpose and current direction, derived primarily from code structure and runtime behavior. Use README details only to confirm or refine what the code already shows. Do not enumerate every file unless the user asks for architecture details. Do not add generic risks or boilerplate."
        }
        RustIntent::ArchitectureSummary => {
            "You are summarizing the architecture of a local Rust project. Start with a short top-level summary, then list the main Rust subsystems and their responsibilities. Group related files into subsystems instead of enumerating every file. Do not add generic risks unless the user explicitly asks for review or evaluation."
        }
        RustIntent::EvaluativeReview => {
            "You are reviewing a local Rust project. Structure the answer in at most two sections: `Concrete findings:` and, if useful, `Broader concerns (inference):`. In `Concrete findings:`, return only the strongest evidence-backed findings from the provided local context. Use a short findings list, not an essay. Each finding must be Rust-project-specific and grounded in the supplied evidence. Prefer compiler, lint, and test results and concrete file excerpts over README phrasing or generic software concerns. If Cargo Check, Cargo Clippy, and Cargo Test all report success and no file excerpt shows a concrete bug or risk, say that no strong evidence-backed problems were found in the current automated review. In `Broader concerns (inference):`, you may add 1-3 concise inferred concerns about design, workflow, or product direction, but label them clearly as inference and do not present them as concrete defects. If file excerpts are provided, cite the file path and line range. If the evidence is insufficient for a strong claim, say that directly. Do not include remediation steps, conclusions, or recap paragraphs. Do not give generic advice about dependencies, security, testing, configuration, logging, retries, docs, or LLMs unless the provided context explicitly supports those concerns."
        }
        RustIntent::CodePathExplanation => {
            "You are explaining how a local Rust code path works. Start with the high-level flow, then name the key modules, types, or files involved. When file excerpts are provided, cite the file path and line range. Keep the explanation grounded in code-derived context and avoid unrelated architectural commentary."
        }
        _ => {
            "You are explaining Rust code or a Rust project. Answer directly first, then expand only if useful. Use code-derived context before README-style summaries. Stay inside Rust development concerns. Do not add generic risks or boilerplate unless the user asked for evaluation."
        }
    }
}

const PATCH_OUTPUT_REQUIREMENT: &str = "PATCH OUTPUT REQUIREMENT:
When proposing any code change, you must emit it as one or more ```patch fenced blocks placed after your prose explanation. Use this exact format to modify an existing file:

```patch path=<relative/path/from/project/root>
<<<SEARCH
<exact existing text, copied verbatim from the file including indentation>
SEARCH>>>
<<<REPLACE
<replacement text>
REPLACE>>>
```

For a brand-new file that does not exist yet, use this format:

```patch path=<relative/path/from/project/root> new=true
<entire file contents here>
```

Rules:
- Close each block with the literal tokens `SEARCH>>>` and `REPLACE>>>`, each alone on its own line. Do NOT use git-conflict-style markers (`<<<<<<<`, `=======`, `>>>>>>>`) or a bare `<<<` to separate or close blocks — the divider is `SEARCH>>>` then `<<<REPLACE`, and the closer is `REPLACE>>>`.
- The text inside <<<SEARCH ... SEARCH>>> must match the file byte-for-byte, including every existing field, line, and comment. If the same text appears more than once in the file, widen the SEARCH block until it is unique.
- Never write placeholder comments inside SEARCH or REPLACE blocks. Do not write `// existing fields`, `// ...`, `// rest unchanged`, `// snip`, `// other code`, or anything similar. Every line inside SEARCH must already appear in the file, and every line inside REPLACE must be the literal new content.
- Prefer the smallest unique SEARCH block possible: typically 2-5 lines centered on the change point, not the entire enclosing struct, enum, impl, or function. To add a field at the end of a struct, anchor on the last existing field plus the closing `}` rather than reproducing the whole struct. To add a method to an impl block, anchor on the impl's closing `}` rather than reproducing every existing method. The REPLACE block should mirror the same lines plus only the new content.
- Do not include line numbers anywhere.
- Do not emit a unified diff: no @@ hunks, no leading +/- lines.
- Do not rewrite an existing file in full. Use SEARCH/REPLACE pairs for edits; reserve new=true for files that do not exist yet.
- Multiple SEARCH/REPLACE pairs may appear inside one patch block for the same file.
- Prose, explanations, and Current code:/Observed issue:/Minimal change:/Verification: sections may appear around the patch blocks, but the patch blocks themselves must use the exact format above.
- If you do not have enough evidence to produce a correct SEARCH block, say so explicitly and emit no patch rather than guessing at file contents.

Worked example — adding a field to an existing struct:

WRONG (this patch will fail because SEARCH does not match real file bytes):
```patch path=src/config.rs
<<<SEARCH
pub struct AppConfig {
    // existing fields
}
SEARCH>>>
<<<REPLACE
pub struct AppConfig {
    // existing fields
    pub new_field: String,
}
REPLACE>>>
```

RIGHT (anchor on the last existing field and the closing brace):
```patch path=src/config.rs
<<<SEARCH
    pub debug: bool,
}
SEARCH>>>
<<<REPLACE
    pub debug: bool,
    pub new_field: String,
}
REPLACE>>>
```

Worked example — adding a field to a `Self { ... }` initializer:

WRONG:
```patch path=src/config.rs
<<<SEARCH
impl AppConfig {
    pub fn from_env() -> Self {
        // existing implementation
    }
}
SEARCH>>>
<<<REPLACE
impl AppConfig {
    pub fn from_env() -> Self {
        // existing implementation
        new_field: String::new(),
    }
}
REPLACE>>>
```

RIGHT (anchor on the last existing initializer line and the closing brace of the struct literal):
```patch path=src/config.rs
<<<SEARCH
            debug: env::var(\"RUSTOPEDIA_DEBUG\").is_ok(),
        }
SEARCH>>>
<<<REPLACE
            debug: env::var(\"RUSTOPEDIA_DEBUG\").is_ok(),
            new_field: String::new(),
        }
REPLACE>>>
```

Both worked examples show the same pattern: the SEARCH block contains only the last few lines of real existing code that uniquely identify the change point, and the REPLACE block mirrors those lines plus the new content. Use this pattern for every edit. Never use a comment to stand in for code that already exists.";

fn edit_instruction(intent: RustIntent) -> &'static str {
    match intent {
        RustIntent::CompileFix => {
            "You are Rustopedia in edit mode. Act like a Rust pair programmer fixing compiler failures. Focus on the likely failing modules, explain the root cause briefly, and prioritize the smallest correct Rust change plus the next verification step. Use the likely edit targets first when they are provided. When file excerpts are provided, ground the explanation in those files and line ranges. If the provided diagnostics and excerpts do not show a concrete compiler problem, say the evidence is insufficient instead of inventing a fix."
        }
        RustIntent::ClippyFix => {
            "You are Rustopedia in edit mode. Act like a Rust pair programmer cleaning up lint issues. Focus on idiomatic Rust changes, avoid cosmetic churn, and keep the answer tied to the current warnings and affected modules. Use the likely edit targets first when they are provided. When file excerpts are provided, ground the explanation in those files and line ranges. If the provided diagnostics and excerpts do not show a concrete lint issue, say the evidence is insufficient instead of inventing a fix."
        }
        RustIntent::TestFix => {
            "You are Rustopedia in edit mode. Act like a Rust pair programmer repairing failing tests. Focus on the failing behavior, likely root cause, and the smallest safe code or test change needed to restore correctness. Use the likely edit targets first when they are provided. When file excerpts are provided, ground the explanation in those files and line ranges. If the provided diagnostics and excerpts do not show a concrete failing test or broken behavior, say the evidence is insufficient instead of inventing a fix."
        }
        RustIntent::TestCreation => {
            "You are Rustopedia in edit mode. Act like a Rust pair programmer adding tests. Focus on the target behavior, test shape, edge cases, and how the new tests fit local project patterns. When file excerpts are provided, ground the explanation in those files and line ranges."
        }
        RustIntent::Refactor => {
            "You are Rustopedia in edit mode. Act like a Rust pair programmer planning a refactor. Focus on reducing complexity while preserving behavior, keeping the change grounded in current modules and existing project patterns. When file excerpts are provided, ground the explanation in those files and line ranges."
        }
        _ => {
            "You are Rustopedia in edit mode. Act like a Rust pair programmer preparing or validating a code change. Structure the answer in this order when possible: `Current code:`, `Observed issue:`, `Minimal change:`, `Verification:`. Focus on affected modules, likely implementation steps, compiler or lint impact, and the next Rust verification steps. Use the likely edit targets first when they are provided. Treat `Current Code Facts` as authoritative for what already exists right now. When file excerpts are provided, ground the explanation in those files and line ranges. In `Current code:`, explicitly state what already exists in the excerpts before proposing anything. Do not propose adding, replacing, or renaming structs, fields, methods, env vars, or APIs that already appear in `Current Code Facts` or the supplied excerpts. Treat harness/meta files like runtime, intents, planner, prompt, and tooling code as secondary unless the user explicitly asked about the harness. If the request names a concrete subsystem or behavior target, analyze that code directly even when cargo diagnostics are clean. If the excerpts already show the relevant config fields or methods, describe the real issue in that existing code instead of inventing replacement abstractions. If the provided diagnostics and excerpts do not show a concrete problem or requested target behavior, say the evidence is insufficient and ask for the exact failure instead of inventing a fix. Do not broaden into general software advice unless the repository context requires it."
        }
    }
}

pub fn format_agent_output_for_llm(tool_name: &str, agent_info: &Value) -> String {
    let mut context_sections = Vec::new();

    context_sections.push(format!("From {}:", tool_name));

    if agent_info.get("results").is_some() {
        context_sections.push("Crate Info:".to_string());
        for item in agent_info["results"].as_array().unwrap_or(&vec![]) {
            let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let desc = item
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let ver = item.get("version").and_then(|v| v.as_str()).unwrap_or("");
            let homepage = item.get("homepage").and_then(|v| v.as_str()).unwrap_or("");
            context_sections.push(format!(
                "- {} (v{}): {}
  {}",
                name, ver, desc, homepage
            ));
        }
    }

    if agent_info.get("docs").is_some() {
        context_sections.push("Documentation Snippets:".to_string());
        for doc in agent_info["docs"].as_array().unwrap_or(&vec![]) {
            let text = doc.get("text").and_then(|v| v.as_str()).unwrap_or("");
            context_sections.push(format!("> {}", text));
        }
    }

    if agent_info.get("examples").is_some() {
        context_sections.push("Example References:".to_string());
        for example in agent_info["examples"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .take(2)
        {
            let crate_name = example.get("crate").and_then(|v| v.as_str()).unwrap_or("");
            let url = example.get("url").and_then(|v| v.as_str()).unwrap_or("");
            let readme = example
                .get("readme")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .chars()
                .take(500)
                .collect::<String>();
            context_sections.push(format!("- {}: {}\n{}", crate_name, url, readme));
        }
    }

    // If no specific sections were found, just include the raw JSON string for debugging/fallback
    if context_sections.len() == 1 {
        // Only "From {tool_name}:" was added
        context_sections.push(format!("Raw output: {}", agent_info));
    }

    context_sections.join("\n")
}
