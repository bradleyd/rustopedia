//! Patch format parser for edit-mode model output.
//!
//! Canonical format the model is asked to produce:
//!
//! ```text
//! ```patch path=src/config.rs
//! <<<SEARCH
//! pub model_name: String,
//! SEARCH>>>
//! <<<REPLACE
//! pub model_name: String,
//! pub edit_model_name: String,
//! REPLACE>>>
//! ```
//! ```
//!
//! For new files:
//!
//! ```text
//! ```patch path=src/patch.rs new=true
//! pub struct Patch { ... }
//! ```
//! ```
//!
//! Parsing is intentionally strict in this first pass so we can observe how
//! often the model produces the canonical shape on its own. Forgiving
//! parsing is a follow-up once we have real failure samples.

use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Patch {
    Modify {
        path: String,
        edits: Vec<SearchReplaceEdit>,
    },
    Create {
        path: String,
        content: String,
    },
    /// Edits that name a Rust item by selector instead of transcribing anchor bytes.
    /// The item's span is resolved with `syn` at verify time.
    Symbolic {
        path: String,
        edits: Vec<SymbolicEdit>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchReplaceEdit {
    pub search: String,
    pub replace: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolicOp {
    Replace,
    InsertAfter,
    InsertBefore,
}

/// Names a top-level Rust item (or impl method) for a symbolic edit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemSelector {
    Struct(String),
    Enum(String),
    Union(String),
    TypeAlias(String),
    Trait(String),
    Fn(String),
    InherentImpl { ty: String },
    TraitImpl { trait_path: String, ty: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicEdit {
    pub op: SymbolicOp,
    pub selector: ItemSelector,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatchParseError {
    pub message: String,
    pub block_excerpt: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParsedPatches {
    pub patches: Vec<Patch>,
    pub errors: Vec<PatchParseError>,
}

impl ParsedPatches {
    pub fn is_empty(&self) -> bool {
        self.patches.is_empty() && self.errors.is_empty()
    }
}

pub fn parse(model_output: &str) -> ParsedPatches {
    let mut result = ParsedPatches::default();
    let normalized = model_output.replace("\r\n", "\n");

    for block in extract_patch_blocks(&normalized) {
        match parse_block(&block.header, &block.body) {
            Ok(patch) => result.patches.push(patch),
            Err(message) => result.errors.push(PatchParseError {
                message,
                block_excerpt: excerpt(&block.body),
            }),
        }
    }

    result
}

struct PatchBlock {
    header: String,
    body: String,
}

fn extract_patch_blocks(text: &str) -> Vec<PatchBlock> {
    let mut blocks = Vec::new();
    let mut remaining = text;

    while let Some(start) = remaining.find("```patch") {
        let after_fence = &remaining[start + "```patch".len()..];
        let Some(header_end) = after_fence.find('\n') else {
            break;
        };
        let header = after_fence[..header_end].trim().to_string();
        let body_start = &after_fence[header_end + 1..];
        let Some(body_end) = find_closing_fence(body_start) else {
            break;
        };
        let body = body_start[..body_end].to_string();
        blocks.push(PatchBlock { header, body });
        let consumed = start + "```patch".len() + header_end + 1 + body_end + "```".len();
        remaining = &remaining[consumed.min(remaining.len())..];
    }

    blocks
}

fn find_closing_fence(text: &str) -> Option<usize> {
    let mut search_from = 0;
    loop {
        let rest = &text[search_from..];
        let idx = rest.find("```")?;
        let absolute = search_from + idx;
        let at_line_start = absolute == 0 || text.as_bytes()[absolute - 1] == b'\n';
        if at_line_start {
            return Some(absolute);
        }
        search_from = absolute + 3;
        if search_from >= text.len() {
            return None;
        }
    }
}

fn parse_block(header: &str, body: &str) -> Result<Patch, String> {
    let attrs = parse_header_attrs(header)?;
    let path = attrs
        .path
        .ok_or_else(|| "patch block is missing path= attribute".to_string())?;

    if attrs.is_new && attrs.edit_symbolic {
        return Err("a patch block cannot be both new=true and edit=symbolic".to_string());
    }

    if attrs.edit_symbolic {
        let edits = parse_symbolic_edits(body)?;
        if edits.is_empty() {
            return Err("symbolic patch block has no @-operations".to_string());
        }
        return Ok(Patch::Symbolic { path, edits });
    }

    if attrs.is_new {
        let content = strip_trailing_newline(body);
        return Ok(Patch::Create {
            path,
            content: content.to_string(),
        });
    }

    let edits = parse_search_replace_edits(body)?;
    if edits.is_empty() {
        return Err(
            "patch block has no SEARCH/REPLACE pairs and is not marked new=true".to_string(),
        );
    }

    Ok(Patch::Modify { path, edits })
}

#[derive(Default)]
struct HeaderAttrs {
    path: Option<String>,
    is_new: bool,
    edit_symbolic: bool,
}

fn parse_header_attrs(header: &str) -> Result<HeaderAttrs, String> {
    let mut attrs = HeaderAttrs::default();
    for token in header.split_whitespace() {
        if let Some((key, value)) = token.split_once('=') {
            match key {
                "path" => {
                    let cleaned = value.trim_matches(|c: char| c == '"' || c == '\'');
                    if cleaned.is_empty() {
                        return Err("patch header has empty path=".to_string());
                    }
                    attrs.path = Some(normalize_path(cleaned));
                }
                "new" => {
                    attrs.is_new = matches!(value.to_ascii_lowercase().as_str(), "true" | "1");
                }
                "edit" => match value.to_ascii_lowercase().as_str() {
                    "symbolic" => attrs.edit_symbolic = true,
                    other => {
                        return Err(format!(
                            "unknown edit mode '{other}' (expected edit=symbolic)"
                        ));
                    }
                },
                other => {
                    return Err(format!("unknown patch header attribute '{other}'"));
                }
            }
        } else if token == "new" {
            attrs.is_new = true;
        } else {
            return Err(format!("unexpected token in patch header: '{token}'"));
        }
    }
    Ok(attrs)
}

fn normalize_path(raw: &str) -> String {
    raw.trim_start_matches("./").to_string()
}

fn parse_search_replace_edits(body: &str) -> Result<Vec<SearchReplaceEdit>, String> {
    let mut edits = Vec::new();
    let lines: Vec<&str> = body.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        if lines[i].trim().is_empty() {
            i += 1;
            continue;
        }

        if !is_search_open(lines[i]) {
            return Err(format!(
                "expected '<<<SEARCH' on its own line, got: '{}'",
                lines[i]
            ));
        }
        i += 1;

        // SEARCH body — ends at the search/replace boundary marker.
        let search_start = i;
        while i < lines.len() && !is_search_close(lines[i]) {
            i += 1;
        }
        if i >= lines.len() {
            return Err("SEARCH block was not closed with 'SEARCH>>>'".to_string());
        }
        let search = join_block(&lines[search_start..i]);

        // Consume the boundary. Canonical form is `SEARCH>>>` followed by a separate
        // `<<<REPLACE`. Models trained on the git-conflict dialect collapse this into a
        // bare `<<<REPLACE` divider with no `SEARCH>>>`; accept that too.
        let boundary = lines[i].trim();
        i += 1;
        if boundary == "<<<REPLACE" {
            eprintln!(
                "[parser warning] SEARCH/REPLACE divider used a bare '<<<REPLACE' without a 'SEARCH>>>' close (forgiving recovery applied)"
            );
        } else {
            if boundary == "REPLACE>>>" {
                eprintln!(
                    "[parser warning] SEARCH block closed with 'REPLACE>>>' instead of 'SEARCH>>>' (forgiving recovery applied)"
                );
            }
            while i < lines.len() && lines[i].trim().is_empty() {
                i += 1;
            }
            if i >= lines.len() || !is_replace_open(lines[i]) {
                return Err("expected '<<<REPLACE' after SEARCH close marker".to_string());
            }
            i += 1;
        }

        // REPLACE body — ends at a replace close marker (any supported dialect) or the
        // start of the next edit.
        let replace_start = i;
        while i < lines.len() && !is_replace_close(lines[i]) && !is_search_open(lines[i]) {
            i += 1;
        }
        if i >= lines.len() {
            return Err("REPLACE block was not closed with 'REPLACE>>>'".to_string());
        }
        let replace = join_block(&lines[replace_start..i]);
        if is_replace_close(lines[i]) {
            if lines[i].trim() == "SEARCH>>>" {
                eprintln!(
                    "[parser warning] REPLACE block closed with 'SEARCH>>>' instead of 'REPLACE>>>' (forgiving recovery applied)"
                );
            }
            i += 1;
        }
        // else: stopped at the next `<<<SEARCH`; leave it for the next iteration.

        edits.push(SearchReplaceEdit { search, replace });
    }

    Ok(edits)
}

const SYMBOLIC_BODY_TERMINATOR: &str = "@@@";

/// Parse a `edit=symbolic` block body: a sequence of `@<op> <selector>` lines, each
/// followed by a body terminated by a line that trims to `@@@`.
fn parse_symbolic_edits(body: &str) -> Result<Vec<SymbolicEdit>, String> {
    let mut edits = Vec::new();
    let lines: Vec<&str> = body.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        if lines[i].trim().is_empty() {
            i += 1;
            continue;
        }

        let op_line = lines[i].trim();
        let Some(rest) = op_line.strip_prefix('@') else {
            return Err(format!(
                "expected an @-operation line (@replace/@after/@before), got: '{op_line}'"
            ));
        };
        if rest.trim() == "@@" {
            return Err("stray '@@@' with no preceding @-operation".to_string());
        }
        let (op_word, selector_str) = rest.split_once(char::is_whitespace).ok_or_else(|| {
            format!("operation '@{rest}' is missing a selector (e.g. @replace struct Foo)")
        })?;
        let op = match op_word {
            "replace" => SymbolicOp::Replace,
            "after" => SymbolicOp::InsertAfter,
            "before" => SymbolicOp::InsertBefore,
            other => {
                return Err(format!(
                    "unknown symbolic operation '@{other}' (expected @replace, @after, or @before)"
                ));
            }
        };
        let selector = parse_selector(selector_str.trim())?;

        i += 1;
        let body_start = i;
        while i < lines.len() && lines[i].trim() != SYMBOLIC_BODY_TERMINATOR {
            i += 1;
        }
        if i >= lines.len() {
            return Err(format!(
                "@{op_word} body was not terminated by a '{SYMBOLIC_BODY_TERMINATOR}' line"
            ));
        }
        let edit_body = join_block(&lines[body_start..i]);
        i += 1; // consume the terminator

        edits.push(SymbolicEdit {
            op,
            selector,
            body: edit_body,
        });
    }

    Ok(edits)
}

/// Parse a selector like `struct Foo`, `fn bar`, `impl Foo`, or `impl Trait for Foo`.
fn parse_selector(text: &str) -> Result<ItemSelector, String> {
    let text = text.trim();
    let (kind, name) = text
        .split_once(char::is_whitespace)
        .map(|(k, n)| (k, n.trim()))
        .ok_or_else(|| {
            format!("selector '{text}' must be '<kind> <name>' (e.g. struct Foo, impl Trait for Foo)")
        })?;

    let ident_ok = |n: &str| !n.is_empty();
    match kind {
        "struct" if ident_ok(name) => Ok(ItemSelector::Struct(name.to_string())),
        "enum" if ident_ok(name) => Ok(ItemSelector::Enum(name.to_string())),
        "union" if ident_ok(name) => Ok(ItemSelector::Union(name.to_string())),
        "type" if ident_ok(name) => Ok(ItemSelector::TypeAlias(name.to_string())),
        "trait" if ident_ok(name) => Ok(ItemSelector::Trait(name.to_string())),
        "fn" if ident_ok(name) => Ok(ItemSelector::Fn(name.to_string())),
        "impl" if ident_ok(name) => {
            if let Some((trait_path, ty)) = name.split_once(" for ") {
                Ok(ItemSelector::TraitImpl {
                    trait_path: strip_ws(trait_path),
                    ty: strip_ws(ty),
                })
            } else {
                Ok(ItemSelector::InherentImpl {
                    ty: strip_ws(name),
                })
            }
        }
        _ => Err(format!(
            "unknown selector kind in '{text}' (allowed: struct, enum, union, type, trait, fn, impl)"
        )),
    }
}

/// Strip all whitespace so impl type/trait comparisons are insensitive to formatting
/// (the selector's `Foo<Bar>` vs syn/quote's `Foo < Bar >`).
fn strip_ws(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn is_search_open(line: &str) -> bool {
    matches!(line.trim(), "<<<SEARCH" | "<<<<<<< SEARCH")
}

/// Boundary between SEARCH and REPLACE bodies. Canonically `SEARCH>>>`; we also accept a
/// bare `<<<REPLACE` divider, and tolerate a swapped `REPLACE>>>`.
fn is_search_close(line: &str) -> bool {
    matches!(line.trim(), "SEARCH>>>" | "REPLACE>>>" | "<<<REPLACE")
}

fn is_replace_open(line: &str) -> bool {
    line.trim() == "<<<REPLACE"
}

/// End of a REPLACE body. Canonically `REPLACE>>>`; we also accept the git/aider-style
/// closers some Rust-trained models emit (`<<<`, `>>>>>>>`, `>>>>>>> REPLACE`) and a
/// swapped `SEARCH>>>`. None of these are valid standalone Rust source lines.
fn is_replace_close(line: &str) -> bool {
    matches!(
        line.trim(),
        "REPLACE>>>" | "SEARCH>>>" | "<<<" | ">>>>>>>" | ">>>>>>> REPLACE"
    )
}

fn join_block(lines: &[&str]) -> String {
    lines.join("\n")
}

fn strip_trailing_newline(text: &str) -> &str {
    text.strip_suffix('\n').unwrap_or(text)
}

fn excerpt(body: &str) -> String {
    let first_line = body.lines().next().unwrap_or("").trim();
    let cleaned: String = first_line.chars().take(80).collect();
    if first_line.chars().count() > 80 {
        format!("{cleaned}…")
    } else {
        cleaned
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnchorStatus {
    Ok,
    NotFound,
    Ambiguous(usize),
    FileMissing,
    FileReadError(String),
}

impl AnchorStatus {
    pub fn label(&self) -> String {
        match self {
            Self::Ok => "[OK]".to_string(),
            Self::NotFound => "[NOT FOUND in file]".to_string(),
            Self::Ambiguous(n) => format!("[AMBIGUOUS: {n} matches]"),
            Self::FileMissing => "[FILE MISSING]".to_string(),
            Self::FileReadError(err) => format!("[FILE READ ERROR: {err}]"),
        }
    }

    pub fn is_applicable(&self) -> bool {
        matches!(self, Self::Ok)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedEdit {
    pub edit: SearchReplaceEdit,
    pub status: AnchorStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedSymbolicEdit {
    pub edit: SymbolicEdit,
    pub status: AnchorStatus,
    /// 1-based inclusive line range of the resolved target, when `status` is `Ok`.
    pub span: Option<(usize, usize)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifiedPatch {
    Modify {
        path: String,
        edits: Vec<VerifiedEdit>,
    },
    Create {
        path: String,
        content: String,
        file_already_exists: bool,
    },
    Symbolic {
        path: String,
        edits: Vec<VerifiedSymbolicEdit>,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VerifiedPatches {
    pub patches: Vec<VerifiedPatch>,
    pub errors: Vec<PatchParseError>,
}

impl VerifiedPatches {
    pub fn is_empty(&self) -> bool {
        self.patches.is_empty() && self.errors.is_empty()
    }
}

pub fn verify(parsed: &ParsedPatches, project_root: &Path) -> VerifiedPatches {
    let mut out = VerifiedPatches {
        errors: parsed.errors.clone(),
        ..Default::default()
    };

    for patch in &parsed.patches {
        out.patches.push(verify_patch(patch, project_root));
    }

    out
}

fn verify_patch(patch: &Patch, project_root: &Path) -> VerifiedPatch {
    match patch {
        Patch::Modify { path, edits } => {
            let full_path = resolve_path(project_root, path);
            let file_content = match std::fs::read_to_string(&full_path) {
                Ok(content) => Some(content),
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
                Err(err) => {
                    return VerifiedPatch::Modify {
                        path: path.clone(),
                        edits: edits
                            .iter()
                            .map(|edit| VerifiedEdit {
                                edit: edit.clone(),
                                status: AnchorStatus::FileReadError(err.to_string()),
                            })
                            .collect(),
                    };
                }
            };

            let verified = edits
                .iter()
                .map(|edit| {
                    let status = match &file_content {
                        None => AnchorStatus::FileMissing,
                        Some(content) => anchor_status(content, &edit.search),
                    };
                    VerifiedEdit {
                        edit: edit.clone(),
                        status,
                    }
                })
                .collect();

            VerifiedPatch::Modify {
                path: path.clone(),
                edits: verified,
            }
        }
        Patch::Create { path, content } => {
            let full_path = resolve_path(project_root, path);
            VerifiedPatch::Create {
                path: path.clone(),
                content: content.clone(),
                file_already_exists: full_path.exists(),
            }
        }
        Patch::Symbolic { path, edits } => {
            let full_path = resolve_path(project_root, path);
            let file_content = match std::fs::read_to_string(&full_path) {
                Ok(content) => Some(content),
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
                Err(err) => {
                    return VerifiedPatch::Symbolic {
                        path: path.clone(),
                        edits: edits
                            .iter()
                            .map(|edit| VerifiedSymbolicEdit {
                                edit: edit.clone(),
                                status: AnchorStatus::FileReadError(err.to_string()),
                                span: None,
                            })
                            .collect(),
                    };
                }
            };

            let verified = edits
                .iter()
                .map(|edit| {
                    let (status, span) = match &file_content {
                        None => (AnchorStatus::FileMissing, None),
                        Some(content) => match resolve_item_span(content, &edit.selector) {
                            AnchorResolution::Resolved {
                                start_line,
                                end_line,
                            } => (AnchorStatus::Ok, Some((start_line, end_line))),
                            AnchorResolution::NotFound => (AnchorStatus::NotFound, None),
                            AnchorResolution::Ambiguous(n) => (AnchorStatus::Ambiguous(n), None),
                            // A parse failure is surfaced as a retryable error (not a silent
                            // drop); the message carries the syn error for the directive.
                            AnchorResolution::ParseFailed(msg) => {
                                (AnchorStatus::FileReadError(msg), None)
                            }
                        },
                    };
                    VerifiedSymbolicEdit {
                        edit: edit.clone(),
                        status,
                        span,
                    }
                })
                .collect();

            VerifiedPatch::Symbolic {
                path: path.clone(),
                edits: verified,
            }
        }
    }
}

/// Result of locating a named item in a source file (1-based inclusive line range).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnchorResolution {
    Resolved { start_line: usize, end_line: usize },
    NotFound,
    Ambiguous(usize),
    ParseFailed(String),
}

/// Resolve a symbolic selector to the target item's line span using `syn` (the same
/// parser + span-locations approach as the Phase-1 file skeletons).
fn resolve_item_span(content: &str, selector: &ItemSelector) -> AnchorResolution {
    use syn::spanned::Spanned;

    let file = match syn::parse_file(content) {
        Ok(file) => file,
        Err(err) => return AnchorResolution::ParseFailed(err.to_string()),
    };

    // Start line including any outer attributes, so a `@replace` span covers `#[derive]`.
    let start_with_attrs = |attrs: &[syn::Attribute], span: proc_macro2::Span| -> usize {
        attrs
            .iter()
            .map(|a| a.span().start().line)
            .min()
            .unwrap_or(span.start().line)
            .min(span.start().line)
    };

    let mut matches: Vec<(usize, usize)> = Vec::new();
    let mut fn_selector_name: Option<&str> = None;

    for item in &file.items {
        let hit = match (selector, item) {
            (ItemSelector::Struct(n), syn::Item::Struct(it)) => (it.ident == n)
                .then(|| (start_with_attrs(&it.attrs, it.span()), it.span().end().line)),
            (ItemSelector::Enum(n), syn::Item::Enum(it)) => (it.ident == n)
                .then(|| (start_with_attrs(&it.attrs, it.span()), it.span().end().line)),
            (ItemSelector::Union(n), syn::Item::Union(it)) => (it.ident == n)
                .then(|| (start_with_attrs(&it.attrs, it.span()), it.span().end().line)),
            (ItemSelector::TypeAlias(n), syn::Item::Type(it)) => (it.ident == n)
                .then(|| (start_with_attrs(&it.attrs, it.span()), it.span().end().line)),
            (ItemSelector::Trait(n), syn::Item::Trait(it)) => (it.ident == n)
                .then(|| (start_with_attrs(&it.attrs, it.span()), it.span().end().line)),
            (ItemSelector::Fn(n), syn::Item::Fn(it)) => {
                fn_selector_name = Some(n);
                (it.sig.ident == n)
                    .then(|| (start_with_attrs(&it.attrs, it.span()), it.span().end().line))
            }
            (ItemSelector::Fn(n), _) => {
                fn_selector_name = Some(n);
                None
            }
            (ItemSelector::InherentImpl { ty }, syn::Item::Impl(it)) => (it.trait_.is_none()
                && &strip_ws(&type_to_string(&it.self_ty)) == ty)
                .then(|| (start_with_attrs(&it.attrs, it.span()), it.span().end().line)),
            (ItemSelector::TraitImpl { trait_path, ty }, syn::Item::Impl(it)) => it
                .trait_
                .as_ref()
                .filter(|(_, path, _)| &strip_ws(&path_to_string(path)) == trait_path)
                .filter(|_| &strip_ws(&type_to_string(&it.self_ty)) == ty)
                .map(|_| (start_with_attrs(&it.attrs, it.span()), it.span().end().line)),
            _ => None,
        };
        if let Some(span) = hit {
            matches.push(span);
        }
    }

    // `fn name` falls back to impl methods when no top-level fn matched.
    if matches.is_empty()
        && let Some(name) = fn_selector_name
    {
        for item in &file.items {
            if let syn::Item::Impl(it) = item {
                for sub in &it.items {
                    if let syn::ImplItem::Fn(m) = sub
                        && m.sig.ident == name
                    {
                        matches.push((start_with_attrs(&m.attrs, m.span()), m.span().end().line));
                    }
                }
            }
        }
    }

    match matches.len() {
        0 => AnchorResolution::NotFound,
        1 => AnchorResolution::Resolved {
            start_line: matches[0].0,
            end_line: matches[0].1,
        },
        n => AnchorResolution::Ambiguous(n),
    }
}

/// Render a `syn::Type` to a normalized string for selector comparison.
fn type_to_string(ty: &syn::Type) -> String {
    use quote::ToTokens;
    ty.to_token_stream().to_string()
}

fn path_to_string(path: &syn::Path) -> String {
    use quote::ToTokens;
    path.to_token_stream().to_string()
}

fn anchor_status(file_content: &str, search: &str) -> AnchorStatus {
    if search.is_empty() {
        return AnchorStatus::NotFound;
    }
    let count = file_content.matches(search).count();
    match count {
        0 => AnchorStatus::NotFound,
        1 => AnchorStatus::Ok,
        n => AnchorStatus::Ambiguous(n),
    }
}

fn resolve_path(project_root: &Path, relative: &str) -> PathBuf {
    project_root.join(relative)
}

/// Re-render verified patches back into canonical ```patch blocks. Used to show a model
/// its own prior submission when asking for a minimal surgical correction — which also
/// normalizes any lenient/git-conflict dialect it used back into the canonical envelope.
pub fn render_canonical_patches(verified: &VerifiedPatches) -> String {
    let mut blocks = Vec::new();
    for patch in &verified.patches {
        match patch {
            VerifiedPatch::Modify { path, edits } => {
                let mut out = format!("```patch path={path}\n");
                for (n, edit) in edits.iter().enumerate() {
                    if n > 0 {
                        out.push('\n');
                    }
                    out.push_str(&format!(
                        "<<<SEARCH\n{}\nSEARCH>>>\n<<<REPLACE\n{}\nREPLACE>>>\n",
                        edit.edit.search, edit.edit.replace
                    ));
                }
                out.push_str("```");
                blocks.push(out);
            }
            VerifiedPatch::Create { path, content, .. } => {
                blocks.push(format!("```patch path={path} new=true\n{content}\n```"));
            }
            VerifiedPatch::Symbolic { path, edits } => {
                let mut out = format!("```patch path={path} edit=symbolic\n");
                for edit in edits {
                    out.push_str(&format!(
                        "@{} {}\n{}\n@@@\n",
                        render_op(edit.edit.op),
                        render_selector(&edit.edit.selector),
                        edit.edit.body
                    ));
                }
                out.push_str("```");
                blocks.push(out);
            }
        }
    }
    blocks.join("\n\n")
}

fn render_op(op: SymbolicOp) -> &'static str {
    match op {
        SymbolicOp::Replace => "replace",
        SymbolicOp::InsertAfter => "after",
        SymbolicOp::InsertBefore => "before",
    }
}

pub(crate) fn render_selector(selector: &ItemSelector) -> String {
    match selector {
        ItemSelector::Struct(n) => format!("struct {n}"),
        ItemSelector::Enum(n) => format!("enum {n}"),
        ItemSelector::Union(n) => format!("union {n}"),
        ItemSelector::TypeAlias(n) => format!("type {n}"),
        ItemSelector::Trait(n) => format!("trait {n}"),
        ItemSelector::Fn(n) => format!("fn {n}"),
        ItemSelector::InherentImpl { ty } => format!("impl {ty}"),
        ItemSelector::TraitImpl { trait_path, ty } => format!("impl {trait_path} for {ty}"),
    }
}

/// List the symbolic selectors of every top-level item in a source file (used to tell the
/// model the names it can target when a selector fails to resolve). Empty if unparseable.
pub fn list_item_selectors(content: &str) -> Vec<String> {
    let Ok(file) = syn::parse_file(content) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for item in &file.items {
        let sel = match item {
            syn::Item::Struct(it) => Some(format!("struct {}", it.ident)),
            syn::Item::Enum(it) => Some(format!("enum {}", it.ident)),
            syn::Item::Union(it) => Some(format!("union {}", it.ident)),
            syn::Item::Type(it) => Some(format!("type {}", it.ident)),
            syn::Item::Trait(it) => Some(format!("trait {}", it.ident)),
            syn::Item::Fn(it) => Some(format!("fn {}", it.sig.ident)),
            syn::Item::Impl(it) => {
                let ty = strip_ws(&type_to_string(&it.self_ty));
                Some(match &it.trait_ {
                    Some((_, path, _)) => {
                        format!("impl {} for {ty}", strip_ws(&path_to_string(path)))
                    }
                    None => format!("impl {ty}"),
                })
            }
            _ => None,
        };
        if let Some(sel) = sel {
            out.push(sel);
        }
    }
    out
}

pub fn render_preview(verified: &VerifiedPatches) -> Option<String> {
    if verified.is_empty() {
        return None;
    }

    let mut sections = Vec::new();
    sections.push("--- Planned Patches (dry-run, no files written) ---".to_string());

    let mut applicable = 0usize;
    let mut blocked = 0usize;
    for patch in &verified.patches {
        match patch {
            VerifiedPatch::Modify { edits, .. } => {
                for edit in edits {
                    if edit.status.is_applicable() {
                        applicable += 1;
                    } else {
                        blocked += 1;
                    }
                }
            }
            VerifiedPatch::Create {
                file_already_exists,
                ..
            } => {
                if *file_already_exists {
                    blocked += 1;
                } else {
                    applicable += 1;
                }
            }
            VerifiedPatch::Symbolic { edits, .. } => {
                for edit in edits {
                    if edit.status.is_applicable() {
                        applicable += 1;
                    } else {
                        blocked += 1;
                    }
                }
            }
        }
    }

    sections.push(format!(
        "Summary: {applicable} edit(s) would apply cleanly, {blocked} blocked"
    ));

    for (idx, patch) in verified.patches.iter().enumerate() {
        sections.push(render_verified_patch(idx + 1, patch));
    }

    if !verified.errors.is_empty() {
        sections.push("--- Patch Parse Errors ---".to_string());
        for error in &verified.errors {
            sections.push(format!("- {} (near: '{}')", error.message, error.block_excerpt));
        }
    }

    Some(sections.join("\n\n"))
}

const PLACEHOLDER_PATTERNS: &[&str] = &[
    "// existing fields",
    "// existing implementation",
    "// existing methods",
    "// existing variants",
    "// rest unchanged",
    "// other code",
    "// snip",
    "// ...",
    "/* existing",
];

pub fn count_placeholder_hits(verified: &VerifiedPatches) -> usize {
    let mut hits = 0;
    for patch in &verified.patches {
        if let VerifiedPatch::Modify { edits, .. } = patch {
            for edit in edits {
                if PLACEHOLDER_PATTERNS
                    .iter()
                    .any(|p| edit.edit.search.contains(p))
                {
                    hits += 1;
                }
            }
        }
    }
    hits
}

fn render_verified_patch(index: usize, patch: &VerifiedPatch) -> String {
    match patch {
        VerifiedPatch::Modify { path, edits } => {
            let mut lines = vec![format!("[{index}] modify {path} ({} edit(s))", edits.len())];
            for (edit_idx, verified) in edits.iter().enumerate() {
                lines.push(format!(
                    "  edit {} of {}: {}",
                    edit_idx + 1,
                    edits.len(),
                    verified.status.label()
                ));
                for line in verified.edit.search.lines() {
                    lines.push(format!("    - {line}"));
                }
                for line in verified.edit.replace.lines() {
                    lines.push(format!("    + {line}"));
                }
            }
            lines.join("\n")
        }
        VerifiedPatch::Create {
            path,
            content,
            file_already_exists,
        } => {
            let line_count = content.lines().count();
            let preview_limit = 12;
            let status = if *file_already_exists {
                "[BLOCKED: file already exists]"
            } else {
                "[OK]"
            };
            let mut lines = vec![format!(
                "[{index}] create {path} ({line_count} line(s)) {status}"
            )];
            for line in content.lines().take(preview_limit) {
                lines.push(format!("    + {line}"));
            }
            if line_count > preview_limit {
                lines.push(format!(
                    "    + … ({} more line(s) omitted)",
                    line_count - preview_limit
                ));
            }
            lines.join("\n")
        }
        VerifiedPatch::Symbolic { path, edits } => {
            let mut lines = vec![format!(
                "[{index}] symbolic-edit {path} ({} op(s))",
                edits.len()
            )];
            for edit in edits {
                lines.push(format!(
                    "  @{} {}: {}",
                    render_op(edit.edit.op),
                    render_selector(&edit.edit.selector),
                    edit.status.label()
                ));
                for line in edit.edit.body.lines().take(12) {
                    lines.push(format!("    + {line}"));
                }
            }
            lines.join("\n")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_canonical_single_edit() {
        let input = r#"
Some prose here.

```patch path=src/config.rs
<<<SEARCH
pub model_name: String,
SEARCH>>>
<<<REPLACE
pub model_name: String,
pub edit_model_name: String,
REPLACE>>>
```

Trailing prose.
"#;

        let parsed = parse(input);
        assert!(parsed.errors.is_empty(), "errors: {:?}", parsed.errors);
        assert_eq!(parsed.patches.len(), 1);

        match &parsed.patches[0] {
            Patch::Modify { path, edits } => {
                assert_eq!(path, "src/config.rs");
                assert_eq!(edits.len(), 1);
                assert_eq!(edits[0].search, "pub model_name: String,");
                assert_eq!(
                    edits[0].replace,
                    "pub model_name: String,\npub edit_model_name: String,"
                );
            }
            other => panic!("expected Modify, got {other:?}"),
        }
    }

    #[test]
    fn parses_git_conflict_dialect_bare_divider_and_closer() {
        // The Strand-Rust-Coder case: a bare `<<<REPLACE` divider (no `SEARCH>>>`) and a
        // bare `<<<` closer, from a model trained on the git-conflict SEARCH/REPLACE form.
        let input = r#"```patch path=src/llm.rs
<<<SEARCH
struct OpenRouterChatResponse {
    choices: Vec<OpenRouterChoice>,
}
<<<REPLACE
impl Foo for OpenRouterChatResponse {}
<<<
```"#;

        let parsed = parse(input);
        assert!(parsed.errors.is_empty(), "errors: {:?}", parsed.errors);
        assert_eq!(parsed.patches.len(), 1);
        match &parsed.patches[0] {
            Patch::Modify { path, edits } => {
                assert_eq!(path, "src/llm.rs");
                assert_eq!(edits.len(), 1);
                assert_eq!(
                    edits[0].search,
                    "struct OpenRouterChatResponse {\n    choices: Vec<OpenRouterChoice>,\n}"
                );
                assert_eq!(edits[0].replace, "impl Foo for OpenRouterChatResponse {}");
            }
            other => panic!("expected Modify, got {other:?}"),
        }
    }

    #[test]
    fn parses_aider_style_replace_closer() {
        let input = r#"```patch path=src/foo.rs
<<<SEARCH
let a = 1;
SEARCH>>>
<<<REPLACE
let a = 2;
>>>>>>> REPLACE
```"#;

        let parsed = parse(input);
        assert!(parsed.errors.is_empty(), "errors: {:?}", parsed.errors);
        assert_eq!(parsed.patches.len(), 1);
        match &parsed.patches[0] {
            Patch::Modify { edits, .. } => {
                assert_eq!(edits[0].search, "let a = 1;");
                assert_eq!(edits[0].replace, "let a = 2;");
            }
            other => panic!("expected Modify, got {other:?}"),
        }
    }

    #[test]
    fn parses_multiple_edits_in_one_block() {
        let input = r#"
```patch path=src/foo.rs
<<<SEARCH
let a = 1;
SEARCH>>>
<<<REPLACE
let a = 2;
REPLACE>>>

<<<SEARCH
let b = 3;
SEARCH>>>
<<<REPLACE
let b = 4;
REPLACE>>>
```
"#;

        let parsed = parse(input);
        assert!(parsed.errors.is_empty());
        assert_eq!(parsed.patches.len(), 1);
        match &parsed.patches[0] {
            Patch::Modify { edits, .. } => assert_eq!(edits.len(), 2),
            other => panic!("expected Modify, got {other:?}"),
        }
    }

    #[test]
    fn parses_new_file_block() {
        let input = r#"
```patch path=src/patch.rs new=true
pub struct Patch {}
REPLACE>>>
```
"#;
        let parsed = parse(input);
        assert!(parsed.errors.is_empty());
        assert_eq!(parsed.patches.len(), 1);
        match &parsed.patches[0] {
            Patch::Create { path, content } => {
                assert_eq!(path, "src/patch.rs");
                assert_eq!(content, "pub struct Patch {}\nREPLACE>>>");
            }
            other => panic!("expected Create, got {other:?}"),
        }
    }

    #[test]
    fn normalizes_leading_dot_slash_in_path() {
        let input = r#"
```patch path=./src/lib.rs
<<<SEARCH
x
SEARCH>>>
<<<REPLACE
y
REPLACE>>>
```
"#;
        let parsed = parse(input);
        assert!(parsed.errors.is_empty());
        match &parsed.patches[0] {
            Patch::Modify { path, .. } => assert_eq!(path, "src/lib.rs"),
            _ => panic!("expected Modify"),
        }
    }

    #[test]
    fn reports_error_for_missing_path() {
        let input = r#"
```patch
<<<SEARCH
x
SEARCH>>>
<<<REPLACE
y
REPLACE>>>
```
"#;
        let parsed = parse(input);
        assert_eq!(parsed.patches.len(), 0);
        assert_eq!(parsed.errors.len(), 1);
        assert!(parsed.errors[0].message.contains("path"));
    }

    #[test]
    fn reports_error_for_unclosed_search() {
        // A genuinely unclosed SEARCH: no divider or close marker of any dialect before
        // the fence ends. (A bare `<<<REPLACE` divider is now forgivingly accepted, so it
        // is covered by parses_git_conflict_dialect_* instead.)
        let input = r#"
```patch path=src/foo.rs
<<<SEARCH
x
y
```
"#;
        let parsed = parse(input);
        assert_eq!(parsed.patches.len(), 0);
        assert_eq!(parsed.errors.len(), 1);
        assert!(parsed.errors[0].message.contains("SEARCH"));
    }

    #[test]
    fn ignores_text_with_no_patch_blocks() {
        let parsed = parse("Just a prose answer, no patches here.");
        assert!(parsed.is_empty());
    }

    struct TempRoot {
        path: PathBuf,
    }

    impl TempRoot {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "rustopedia_patch_test_{}_{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            std::fs::create_dir_all(&path).unwrap();
            Self { path }
        }

        fn write(&self, relative: &str, content: &str) {
            let full = self.path.join(relative);
            if let Some(parent) = full.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::write(full, content).unwrap();
        }
    }

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn render_preview_returns_none_for_empty_verified() {
        let verified = VerifiedPatches::default();
        assert!(render_preview(&verified).is_none());
    }

    #[test]
    fn verify_marks_present_anchor_ok() {
        let root = TempRoot::new();
        root.write("src/foo.rs", "let a = 1;\nlet b = 2;\n");

        let parsed = ParsedPatches {
            patches: vec![Patch::Modify {
                path: "src/foo.rs".to_string(),
                edits: vec![SearchReplaceEdit {
                    search: "let a = 1;".to_string(),
                    replace: "let a = 99;".to_string(),
                }],
            }],
            errors: vec![],
        };

        let verified = verify(&parsed, &root.path);
        match &verified.patches[0] {
            VerifiedPatch::Modify { edits, .. } => {
                assert_eq!(edits[0].status, AnchorStatus::Ok);
            }
            _ => panic!("expected Modify"),
        }
    }

    #[test]
    fn verify_marks_missing_anchor_not_found() {
        let root = TempRoot::new();
        root.write("src/foo.rs", "let a = 1;\n");

        let parsed = ParsedPatches {
            patches: vec![Patch::Modify {
                path: "src/foo.rs".to_string(),
                edits: vec![SearchReplaceEdit {
                    search: "let nonexistent = 0;".to_string(),
                    replace: "x".to_string(),
                }],
            }],
            errors: vec![],
        };

        let verified = verify(&parsed, &root.path);
        match &verified.patches[0] {
            VerifiedPatch::Modify { edits, .. } => {
                assert_eq!(edits[0].status, AnchorStatus::NotFound);
            }
            _ => panic!("expected Modify"),
        }
    }

    #[test]
    fn verify_marks_ambiguous_anchor() {
        let root = TempRoot::new();
        root.write(
            "src/foo.rs",
            "let a = 1;\nlet b = 2;\nlet a = 1;\n",
        );

        let parsed = ParsedPatches {
            patches: vec![Patch::Modify {
                path: "src/foo.rs".to_string(),
                edits: vec![SearchReplaceEdit {
                    search: "let a = 1;".to_string(),
                    replace: "let a = 99;".to_string(),
                }],
            }],
            errors: vec![],
        };

        let verified = verify(&parsed, &root.path);
        match &verified.patches[0] {
            VerifiedPatch::Modify { edits, .. } => {
                assert_eq!(edits[0].status, AnchorStatus::Ambiguous(2));
            }
            _ => panic!("expected Modify"),
        }
    }

    #[test]
    fn verify_marks_missing_file() {
        let root = TempRoot::new();

        let parsed = ParsedPatches {
            patches: vec![Patch::Modify {
                path: "src/missing.rs".to_string(),
                edits: vec![SearchReplaceEdit {
                    search: "anything".to_string(),
                    replace: "else".to_string(),
                }],
            }],
            errors: vec![],
        };

        let verified = verify(&parsed, &root.path);
        match &verified.patches[0] {
            VerifiedPatch::Modify { edits, .. } => {
                assert_eq!(edits[0].status, AnchorStatus::FileMissing);
            }
            _ => panic!("expected Modify"),
        }
    }

    #[test]
    fn verify_flags_create_when_file_already_exists() {
        let root = TempRoot::new();
        root.write("src/existing.rs", "already here\n");

        let parsed = ParsedPatches {
            patches: vec![
                Patch::Create {
                    path: "src/existing.rs".to_string(),
                    content: "new content".to_string(),
                },
                Patch::Create {
                    path: "src/brand_new.rs".to_string(),
                    content: "fresh".to_string(),
                },
            ],
            errors: vec![],
        };

        let verified = verify(&parsed, &root.path);
        match &verified.patches[0] {
            VerifiedPatch::Create {
                file_already_exists, ..
            } => assert!(file_already_exists),
            _ => panic!("expected Create"),
        }
        match &verified.patches[1] {
            VerifiedPatch::Create {
                file_already_exists, ..
            } => assert!(!file_already_exists),
            _ => panic!("expected Create"),
        }
    }

    #[test]
    fn render_preview_includes_anchor_status_and_summary() {
        let root = TempRoot::new();
        root.write("src/foo.rs", "let a = 1;\n");

        let parsed = ParsedPatches {
            patches: vec![
                Patch::Modify {
                    path: "src/foo.rs".to_string(),
                    edits: vec![
                        SearchReplaceEdit {
                            search: "let a = 1;".to_string(),
                            replace: "let a = 2;".to_string(),
                        },
                        SearchReplaceEdit {
                            search: "let nonexistent = 0;".to_string(),
                            replace: "x".to_string(),
                        },
                    ],
                },
                Patch::Create {
                    path: "src/new.rs".to_string(),
                    content: "pub fn hi() {}\n".to_string(),
                },
            ],
            errors: vec![PatchParseError {
                message: "expected '<<<SEARCH'".to_string(),
                block_excerpt: "garbled block".to_string(),
            }],
        };

        let verified = verify(&parsed, &root.path);
        let rendered = render_preview(&verified).expect("expected preview");
        assert!(rendered.contains("[OK]"));
        assert!(rendered.contains("[NOT FOUND in file]"));
        assert!(rendered.contains("modify src/foo.rs"));
        assert!(rendered.contains("create src/new.rs"));
        assert!(rendered.contains("Summary: 2 edit(s) would apply cleanly, 1 blocked"));
        assert!(rendered.contains("Patch Parse Errors"));
    }

    #[test]
    fn render_canonical_patches_round_trips_into_canonical_envelope() {
        let root = TempRoot::new();
        root.write("src/foo.rs", "let a = 1;\n");

        // Parse a patch written in the lenient git-conflict dialect...
        let parsed = parse(
            "```patch path=src/foo.rs\n<<<SEARCH\nlet a = 1;\n<<<REPLACE\nlet a = 2;\n<<<\n```",
        );
        let verified = verify(&parsed, &root.path);

        // ...and confirm it re-renders into the canonical envelope.
        let rendered = render_canonical_patches(&verified);
        assert_eq!(
            rendered,
            "```patch path=src/foo.rs\n<<<SEARCH\nlet a = 1;\nSEARCH>>>\n<<<REPLACE\nlet a = 2;\nREPLACE>>>\n```"
        );
    }

    // ---- Symbolic edit tests ----

    #[test]
    fn parses_symbolic_replace_and_after() {
        let input = "```patch path=src/llm.rs edit=symbolic\n@replace struct Foo\nstruct Foo { x: i32 }\n@@@\n@after struct Foo\nimpl Foo {}\n@@@\n```";
        let parsed = parse(input);
        assert!(parsed.errors.is_empty(), "errors: {:?}", parsed.errors);
        assert_eq!(parsed.patches.len(), 1);
        match &parsed.patches[0] {
            Patch::Symbolic { path, edits } => {
                assert_eq!(path, "src/llm.rs");
                assert_eq!(edits.len(), 2);
                assert_eq!(edits[0].op, SymbolicOp::Replace);
                assert_eq!(edits[0].selector, ItemSelector::Struct("Foo".into()));
                assert_eq!(edits[0].body, "struct Foo { x: i32 }");
                assert_eq!(edits[1].op, SymbolicOp::InsertAfter);
            }
            other => panic!("expected Symbolic, got {other:?}"),
        }
    }

    #[test]
    fn parses_symbolic_impl_trait_selector() {
        let input = "```patch path=p.rs edit=symbolic\n@after impl Deserialize for Foo\nx\n@@@\n```";
        match &parse(input).patches[0] {
            Patch::Symbolic { edits, .. } => assert_eq!(
                edits[0].selector,
                ItemSelector::TraitImpl {
                    trait_path: "Deserialize".into(),
                    ty: "Foo".into()
                }
            ),
            other => panic!("expected Symbolic, got {other:?}"),
        }
    }

    #[test]
    fn symbolic_missing_terminator_and_unknown_kind_error() {
        let no_term = parse("```patch path=p.rs edit=symbolic\n@replace struct Foo\nbody\n```");
        assert!(no_term.patches.is_empty() && !no_term.errors.is_empty());
        assert!(no_term.errors[0].message.contains("@@@"));

        let bad_kind = parse("```patch path=p.rs edit=symbolic\n@replace gizmo Foo\nx\n@@@\n```");
        assert!(bad_kind.errors[0].message.contains("unknown selector kind"));
    }

    #[test]
    fn search_replace_still_parses_without_symbolic_header() {
        // Coexistence: a normal patch block is unaffected.
        match &parse("```patch path=p.rs\n<<<SEARCH\na\nSEARCH>>>\n<<<REPLACE\nb\nREPLACE>>>\n```").patches[0]
        {
            Patch::Modify { .. } => {}
            other => panic!("expected Modify, got {other:?}"),
        }
    }

    #[test]
    fn resolve_struct_span_includes_attrs() {
        let content = "#[derive(Deserialize)]\nstruct Foo {\n    x: i32,\n}\n";
        match resolve_item_span(content, &ItemSelector::Struct("Foo".into())) {
            AnchorResolution::Resolved { start_line, end_line } => {
                assert_eq!(start_line, 1); // the #[derive] line
                assert_eq!(end_line, 4); // the closing brace
            }
            other => panic!("expected Resolved, got {other:?}"),
        }
    }

    #[test]
    fn resolve_fn_falls_back_to_impl_method_and_detects_ambiguity() {
        let one = "struct A;\nimpl A { fn run(&self) {} }\n";
        assert!(matches!(
            resolve_item_span(one, &ItemSelector::Fn("run".into())),
            AnchorResolution::Resolved { .. }
        ));
        let two = "struct A; struct B;\nimpl A { fn run(&self) {} }\nimpl B { fn run(&self) {} }\n";
        assert_eq!(
            resolve_item_span(two, &ItemSelector::Fn("run".into())),
            AnchorResolution::Ambiguous(2)
        );
    }

    #[test]
    fn resolve_not_found_and_parse_failure() {
        assert_eq!(
            resolve_item_span("struct A;\n", &ItemSelector::Struct("Nope".into())),
            AnchorResolution::NotFound
        );
        assert!(matches!(
            resolve_item_span("fn broken( {", &ItemSelector::Fn("broken".into())),
            AnchorResolution::ParseFailed(_)
        ));
    }

    #[test]
    fn resolve_trait_impl_matches_self_type_and_trait() {
        let content = "struct Foo;\nimpl Bar for Foo {}\nimpl Baz for Foo {}\n";
        assert!(matches!(
            resolve_item_span(
                content,
                &ItemSelector::TraitImpl { trait_path: "Bar".into(), ty: "Foo".into() }
            ),
            AnchorResolution::Resolved { .. }
        ));
    }

    #[test]
    fn verify_symbolic_parse_failure_is_retryable_filereaderror() {
        let root = TempRoot::new();
        root.write("src/foo.rs", "fn broken( {\n"); // unparseable
        let parsed = ParsedPatches {
            patches: vec![Patch::Symbolic {
                path: "src/foo.rs".into(),
                edits: vec![SymbolicEdit {
                    op: SymbolicOp::Replace,
                    selector: ItemSelector::Struct("Foo".into()),
                    body: "struct Foo;".into(),
                }],
            }],
            errors: vec![],
        };
        let verified = verify(&parsed, &root.path);
        match &verified.patches[0] {
            VerifiedPatch::Symbolic { edits, .. } => {
                assert!(matches!(edits[0].status, AnchorStatus::FileReadError(_)));
                assert!(!edits[0].status.is_applicable());
            }
            other => panic!("expected Symbolic, got {other:?}"),
        }
    }

    #[test]
    fn render_canonical_symbolic_round_trips() {
        let root = TempRoot::new();
        root.write("src/foo.rs", "struct Foo {\n    x: i32,\n}\n");
        let parsed = parse(
            "```patch path=src/foo.rs edit=symbolic\n@replace struct Foo\nstruct Foo {\n    x: i64,\n}\n@@@\n```",
        );
        let verified = verify(&parsed, &root.path);
        let rendered = render_canonical_patches(&verified);
        assert_eq!(
            rendered,
            "```patch path=src/foo.rs edit=symbolic\n@replace struct Foo\nstruct Foo {\n    x: i64,\n}\n@@@\n```"
        );
    }

    #[test]
    fn forgiving_parser_accepts_replace_marker_closing_replace_block() {
        // Real Ollama qwen2.5-coder:14b failure mode: model emitted SEARCH>>>
        // to close the REPLACE block instead of REPLACE>>>.
        let input = r#"
```patch path=src/foo.rs
<<<SEARCH
let a = 1;
SEARCH>>>
<<<REPLACE
let a = 2;
SEARCH>>>
```
"#;
        let parsed = parse(input);
        assert!(parsed.errors.is_empty(), "errors: {:?}", parsed.errors);
        assert_eq!(parsed.patches.len(), 1);
        match &parsed.patches[0] {
            Patch::Modify { edits, .. } => {
                assert_eq!(edits[0].search, "let a = 1;");
                assert_eq!(edits[0].replace, "let a = 2;");
            }
            _ => panic!("expected Modify"),
        }
    }

    #[test]
    fn forgiving_parser_accepts_replace_marker_closing_search_block() {
        // Symmetric case: model used REPLACE>>> to close the SEARCH block.
        let input = r#"
```patch path=src/foo.rs
<<<SEARCH
let a = 1;
REPLACE>>>
<<<REPLACE
let a = 2;
REPLACE>>>
```
"#;
        let parsed = parse(input);
        assert!(parsed.errors.is_empty(), "errors: {:?}", parsed.errors);
        assert_eq!(parsed.patches.len(), 1);
        match &parsed.patches[0] {
            Patch::Modify { edits, .. } => {
                assert_eq!(edits[0].search, "let a = 1;");
                assert_eq!(edits[0].replace, "let a = 2;");
            }
            _ => panic!("expected Modify"),
        }
    }

    #[test]
    fn parses_multiple_blocks_across_files() {
        let input = r#"
```patch path=src/a.rs
<<<SEARCH
1
SEARCH>>>
<<<REPLACE
2
REPLACE>>>
```

Some text.

```patch path=src/b.rs
<<<SEARCH
3
SEARCH>>>
<<<REPLACE
4
REPLACE>>>
```
"#;
        let parsed = parse(input);
        assert!(parsed.errors.is_empty());
        assert_eq!(parsed.patches.len(), 2);
    }
}
