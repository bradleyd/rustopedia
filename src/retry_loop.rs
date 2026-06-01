//! Edit-mode retry loop infrastructure.
//!
//! When patch anchor verification reports `NotFound` or `Ambiguous` for at
//! least one edit, the runtime can issue a retry turn with fresh evidence
//! about the area the model tried to anchor on. The "evidence gatherer"
//! here finds the model's intended anchor in the real file and pulls a
//! wider, asymmetric window so the model can see what comes *after* the
//! line it likely thought was last.

use anyhow::Result;
use std::path::Path;
use tokio::fs;

use crate::config::AppConfig;
use crate::memory::FileExcerpt;
use crate::patch::{AnchorStatus, VerifiedPatch, VerifiedPatches};

const CONTEXT_LINES_BEFORE: usize = 5;
const CONTEXT_LINES_AFTER: usize = 60;
const MAX_AMBIGUOUS_SLICES: usize = 3;

#[derive(Debug, Clone)]
pub struct AnchorRetryHit {
    pub path: String,
    pub edit_index: usize,
    pub edit_total: usize,
    pub failed_search: String,
    pub status: AnchorStatus,
    pub diagnosis: RetryDiagnosis,
}

#[derive(Debug, Clone)]
pub enum RetryDiagnosis {
    AnchorLineFound {
        anchor_line_no: usize,
        anchor_line_text: String,
        file_slice: FileExcerpt,
    },
    AnchorLineMissing,
    MultipleMatches {
        match_lines: Vec<usize>,
        file_slices: Vec<FileExcerpt>,
    },
    CreateOnExistingFile {
        file_head: FileExcerpt,
    },
    FileUnreadable {
        message: String,
    },
    /// The patches applied cleanly but `cargo check` failed against the
    /// resulting tree. Carries the compiler output and, where parseable,
    /// file slices around each failing span (post-patch content).
    CargoCheckFailed {
        output_excerpt: String,
        failing_spans: Vec<FailingSpan>,
    },
}

#[derive(Debug, Clone)]
pub struct FailingSpan {
    pub path: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub file_slice: Option<FileExcerpt>,
}

#[derive(Debug, Clone, Default)]
pub struct RetryEvidence {
    pub hits: Vec<AnchorRetryHit>,
}

impl RetryEvidence {
    pub fn is_empty(&self) -> bool {
        self.hits.is_empty()
    }
}

pub fn has_retryable_failures(verified: &VerifiedPatches) -> bool {
    verified.patches.iter().any(|patch| match patch {
        VerifiedPatch::Modify { edits, .. } => edits.iter().any(|edit| {
            matches!(
                edit.status,
                AnchorStatus::NotFound | AnchorStatus::Ambiguous(_)
            )
        }),
        VerifiedPatch::Create {
            file_already_exists,
            ..
        } => *file_already_exists,
    })
}

pub async fn gather_retry_evidence(
    verified: &VerifiedPatches,
    config: &AppConfig,
) -> Result<RetryEvidence> {
    let mut hits = Vec::new();

    for patch in &verified.patches {
        match patch {
            VerifiedPatch::Modify { path, edits } => {
                let total = edits.len();
                for (idx, edit) in edits.iter().enumerate() {
                    let diagnosis = match &edit.status {
                        AnchorStatus::NotFound => {
                            diagnose_not_found(path, &edit.edit.search, config).await
                        }
                        AnchorStatus::Ambiguous(_) => {
                            diagnose_ambiguous(path, &edit.edit.search, config).await
                        }
                        _ => continue,
                    };

                    hits.push(AnchorRetryHit {
                        path: path.clone(),
                        edit_index: idx,
                        edit_total: total,
                        failed_search: edit.edit.search.clone(),
                        status: edit.status.clone(),
                        diagnosis,
                    });
                }
            }
            VerifiedPatch::Create {
                path,
                file_already_exists,
                ..
            } => {
                if !*file_already_exists {
                    continue;
                }
                let diagnosis = diagnose_create_on_existing(path, config).await;
                hits.push(AnchorRetryHit {
                    path: path.clone(),
                    edit_index: 0,
                    edit_total: 1,
                    failed_search: String::new(),
                    status: AnchorStatus::NotFound, // not really meaningful for Create; rendering branches on diagnosis
                    diagnosis,
                });
            }
        }
    }

    Ok(RetryEvidence { hits })
}

/// Build retry evidence from a failed `cargo check` run inside a scratch
/// overlay. `overlay_root` should be the directory the check ran against
/// (used to pull file slices around each failing span so the model sees its
/// own broken code, not the pre-patch original).
pub async fn gather_validation_evidence(
    output_excerpt: &str,
    overlay_root: &Path,
) -> RetryEvidence {
    let failing_spans = parse_short_diagnostics(output_excerpt, overlay_root).await;
    RetryEvidence {
        hits: vec![AnchorRetryHit {
            path: String::new(),
            edit_index: 0,
            edit_total: 1,
            failed_search: String::new(),
            status: AnchorStatus::NotFound, // not meaningful for validation; rendering branches on diagnosis
            diagnosis: RetryDiagnosis::CargoCheckFailed {
                output_excerpt: output_excerpt.to_string(),
                failing_spans,
            },
        }],
    }
}

async fn parse_short_diagnostics(output: &str, overlay_root: &Path) -> Vec<FailingSpan> {
    let mut spans = Vec::new();
    for line in output.lines() {
        let Some(span) = parse_short_diagnostic_line(line) else {
            continue;
        };
        let file_slice = read_file_slice_for_span(overlay_root, &span).await;
        spans.push(FailingSpan { file_slice, ..span });
        if spans.len() >= 6 {
            break;
        }
    }
    spans
}

fn parse_short_diagnostic_line(line: &str) -> Option<FailingSpan> {
    let line = line.trim();
    // Format: "<path>:<line>:<col>: error[CODE]: msg" or "...: error: msg" or "...: warning: msg"
    // We only retry on errors.
    let mut parts = line.splitn(4, ':');
    let path = parts.next()?.to_string();
    let line_no: usize = parts.next()?.parse().ok()?;
    let col_no: usize = parts.next()?.parse().ok()?;
    let tail = parts.next()?.trim();
    if !tail.starts_with("error") {
        return None;
    }
    if path.is_empty() || path.starts_with("warning") || path.starts_with("error") {
        return None;
    }
    Some(FailingSpan {
        path,
        line: line_no,
        column: col_no,
        message: tail.to_string(),
        file_slice: None,
    })
}

async fn read_file_slice_for_span(overlay_root: &Path, span: &FailingSpan) -> Option<FileExcerpt> {
    let full = overlay_root.join(&span.path);
    let content = fs::read_to_string(&full).await.ok()?;
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return None;
    }
    let slice = slice_window(
        &span.path,
        &lines,
        span.line,
        CONTEXT_LINES_BEFORE,
        CONTEXT_LINES_AFTER.min(40),
    );
    Some(slice)
}

async fn diagnose_create_on_existing(relative_path: &str, config: &AppConfig) -> RetryDiagnosis {
    let full_path = Path::new(&config.project_root).join(relative_path);
    let file_content = match fs::read_to_string(&full_path).await {
        Ok(content) => content,
        Err(err) => {
            return RetryDiagnosis::FileUnreadable {
                message: err.to_string(),
            };
        }
    };

    let lines: Vec<&str> = file_content.lines().collect();
    let head_end = lines.len().min(CONTEXT_LINES_AFTER);
    let head_text = lines
        .get(0..head_end)
        .unwrap_or(&[])
        .join("\n");

    RetryDiagnosis::CreateOnExistingFile {
        file_head: FileExcerpt {
            path: relative_path.to_string(),
            start_line: 1,
            end_line: head_end.max(1),
            text: head_text,
        },
    }
}

async fn diagnose_not_found(
    relative_path: &str,
    search: &str,
    config: &AppConfig,
) -> RetryDiagnosis {
    let full_path = Path::new(&config.project_root).join(relative_path);
    let file_content = match fs::read_to_string(&full_path).await {
        Ok(content) => content,
        Err(err) => {
            return RetryDiagnosis::FileUnreadable {
                message: err.to_string(),
            };
        }
    };

    let Some(first_line) = first_nonempty_line(search) else {
        return RetryDiagnosis::AnchorLineMissing;
    };
    let normalized_anchor = first_line.trim_end();

    let lines: Vec<&str> = file_content.lines().collect();
    let mut found_at: Option<usize> = None;
    for (i, line) in lines.iter().enumerate() {
        if line.trim_end() == normalized_anchor {
            found_at = Some(i + 1);
            break;
        }
    }

    let Some(line_no) = found_at else {
        return RetryDiagnosis::AnchorLineMissing;
    };

    let slice = slice_window(
        relative_path,
        &lines,
        line_no,
        CONTEXT_LINES_BEFORE,
        CONTEXT_LINES_AFTER,
    );

    RetryDiagnosis::AnchorLineFound {
        anchor_line_no: line_no,
        anchor_line_text: normalized_anchor.to_string(),
        file_slice: slice,
    }
}

async fn diagnose_ambiguous(
    relative_path: &str,
    search: &str,
    config: &AppConfig,
) -> RetryDiagnosis {
    let full_path = Path::new(&config.project_root).join(relative_path);
    let file_content = match fs::read_to_string(&full_path).await {
        Ok(content) => content,
        Err(err) => {
            return RetryDiagnosis::FileUnreadable {
                message: err.to_string(),
            };
        }
    };

    let lines: Vec<&str> = file_content.lines().collect();
    let mut match_lines = Vec::new();
    let mut start = 0;
    while let Some(idx) = file_content[start..].find(search) {
        let absolute = start + idx;
        let line_no = file_content[..absolute].matches('\n').count() + 1;
        match_lines.push(line_no);
        start = absolute + search.len().max(1);
        if match_lines.len() >= MAX_AMBIGUOUS_SLICES {
            break;
        }
    }

    let file_slices = match_lines
        .iter()
        .map(|line_no| {
            slice_window(
                relative_path,
                &lines,
                *line_no,
                CONTEXT_LINES_BEFORE,
                CONTEXT_LINES_AFTER,
            )
        })
        .collect();

    RetryDiagnosis::MultipleMatches {
        match_lines,
        file_slices,
    }
}

pub fn build_retry_directive(original_query: &str, evidence: &RetryEvidence) -> String {
    if evidence.is_empty() {
        return String::new();
    }

    let mut sections = vec![format!(
        "Previous attempt's patches did not match the real file. Failures and the real file content are listed below.\n\nTask remains: {}\n\nEmit the corrected SEARCH/REPLACE patch blocks immediately. Keep prose to 1-2 sentences maximum. Anchor each SEARCH on text that literally appears in the file slices shown. The evidence is here — do not refuse on \"insufficient evidence\" grounds.",
        original_query.trim()
    )];

    for (n, hit) in evidence.hits.iter().enumerate() {
        sections.push(render_hit(n + 1, hit));
    }

    sections.push(
        "Output the corrected patch block(s) first. Minimal prose only.".to_string(),
    );

    sections.join("\n\n")
}

fn render_hit(index: usize, hit: &AnchorRetryHit) -> String {
    let is_create_on_existing = matches!(
        hit.diagnosis,
        RetryDiagnosis::CreateOnExistingFile { .. }
    );
    let is_cargo_check_failure = matches!(
        hit.diagnosis,
        RetryDiagnosis::CargoCheckFailed { .. }
    );

    let header = if is_create_on_existing {
        format!(
            "Failure #{index}: {} — status: [BLOCKED: you used new=true but the file already exists]",
            hit.path
        )
    } else if is_cargo_check_failure {
        format!("Failure #{index}: cargo check failed after applying your patch")
    } else {
        format!(
            "Failure #{index}: {} (edit {} of {}) — status: {}",
            hit.path,
            hit.edit_index + 1,
            hit.edit_total,
            hit.status.label()
        )
    };

    let failed = if hit.failed_search.is_empty() {
        String::new()
    } else {
        format!(
            "Your SEARCH text was:\n```\n{}\n```",
            hit.failed_search.trim_end()
        )
    };

    let diagnosis = match &hit.diagnosis {
        RetryDiagnosis::AnchorLineFound {
            anchor_line_no,
            anchor_line_text,
            file_slice,
        } => format!(
            "The first line of your SEARCH (`{}`) was found in the file at line {anchor_line_no}, but the rest of your SEARCH does not match the surrounding bytes. Actual file content of {} (lines {}-{}):\n```\n{}\n```\nRe-anchor on text that actually appears in this slice. Prefer the LAST existing line plus the actual closing brace, not the line nearest to where you guessed.",
            anchor_line_text,
            file_slice.path,
            file_slice.start_line,
            file_slice.end_line,
            file_slice.text
        ),
        RetryDiagnosis::AnchorLineMissing => {
            "Not a single line of your SEARCH was found in the file. You likely hallucinated this content. Re-read the file evidence already in working memory before producing a new patch.".to_string()
        }
        RetryDiagnosis::MultipleMatches {
            match_lines,
            file_slices,
        } => {
            let lines_list = match_lines
                .iter()
                .map(|l| l.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let slices = file_slices
                .iter()
                .map(|s| {
                    format!(
                        "Context around line {} of {} (lines {}-{}):\n```\n{}\n```",
                        s.start_line + CONTEXT_LINES_BEFORE.min(s.start_line - 1),
                        s.path,
                        s.start_line,
                        s.end_line,
                        s.text
                    )
                })
                .collect::<Vec<_>>()
                .join("\n\n");
            format!(
                "Your SEARCH text matched multiple locations (lines: {lines_list}). The SEARCH block must be unique. Widen the SEARCH to include enough surrounding lines so only one match remains.\n\n{slices}"
            )
        }
        RetryDiagnosis::CreateOnExistingFile { file_head } => {
            format!(
                "You emitted a patch with `new=true` for {}, but that file already exists in the workspace. The `new=true` form is reserved for files that do not exist yet. To modify the existing file, use one or more `<<<SEARCH ... SEARCH>>> <<<REPLACE ... REPLACE>>>` pairs against the real content. Here are the first {} line(s) of the existing file so you can pick a real anchor:\n```\n{}\n```\nRe-anchor your changes on real lines in this file. Do not emit `new=true` again for this path.",
                file_head.path,
                file_head.end_line,
                file_head.text
            )
        }
        RetryDiagnosis::FileUnreadable { message } => {
            format!("The target file could not be read while diagnosing this failure: {message}. Verify the file path.")
        }
        RetryDiagnosis::CargoCheckFailed {
            output_excerpt,
            failing_spans,
        } => {
            let mut sections = vec![format!(
                "Your patches applied cleanly to the file(s), but the resulting code does not compile. The compiler output (truncated) is:\n```\n{}\n```",
                output_excerpt.trim_end()
            )];
            for (n, span) in failing_spans.iter().enumerate() {
                let slice = match &span.file_slice {
                    Some(s) => format!(
                        "Post-patch code around the failure ({} lines {}-{}):\n```\n{}\n```",
                        s.path, s.start_line, s.end_line, s.text
                    ),
                    None => "(could not load post-patch file slice for this span)".to_string(),
                };
                sections.push(format!(
                    "Compiler span #{}: {}:{}:{} — {}\n\n{}",
                    n + 1,
                    span.path,
                    span.line,
                    span.column,
                    span.message,
                    slice
                ));
            }
            sections.push(
                "Re-emit the corrected SEARCH/REPLACE block(s). The previous edits made it into the file, so anchor against the POST-PATCH content shown above, not the pre-patch original. Only invent symbols, fields, or methods that actually exist in the project — check the post-patch slices and the file evidence already in working memory first.".to_string(),
            );
            sections.join("\n\n")
        }
    };

    let parts: Vec<&str> = [header.as_str(), failed.as_str(), diagnosis.as_str()]
        .iter()
        .copied()
        .filter(|s| !s.is_empty())
        .collect();
    parts.join("\n\n")
}

fn first_nonempty_line(text: &str) -> Option<&str> {
    text.lines().find(|line| !line.trim().is_empty())
}

fn slice_window(
    path: &str,
    lines: &[&str],
    center_line: usize,
    before: usize,
    after: usize,
) -> FileExcerpt {
    let start = center_line.saturating_sub(before).max(1);
    let end = (center_line + after).min(lines.len());
    let text = lines
        .get((start - 1)..end)
        .unwrap_or(&[])
        .join("\n");

    FileExcerpt {
        path: path.to_string(),
        start_line: start,
        end_line: end,
        text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_nonempty_line_skips_blanks() {
        assert_eq!(first_nonempty_line("\n\n  hello\nworld"), Some("  hello"));
        assert_eq!(first_nonempty_line(""), None);
    }

    #[test]
    fn slice_window_clamps_to_file_bounds() {
        let lines = vec!["a", "b", "c", "d", "e"];
        let slice = slice_window("x.rs", &lines, 2, 5, 5);
        assert_eq!(slice.start_line, 1);
        assert_eq!(slice.end_line, 5);
        assert_eq!(slice.text, "a\nb\nc\nd\ne");
    }

    #[test]
    fn slice_window_asymmetric_range() {
        let lines: Vec<&str> = (1..=100).map(|_| "line").collect();
        let slice = slice_window("x.rs", &lines, 50, 5, 60);
        assert_eq!(slice.start_line, 45);
        assert_eq!(slice.end_line, 100);
    }

    #[tokio::test]
    async fn diagnose_not_found_locates_anchor_in_real_file() {
        let tmp = std::env::temp_dir().join(format!(
            "rustopedia_retry_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&tmp).unwrap();
        let file = tmp.join("foo.rs");
        std::fs::write(
            &file,
            "line 1\nline 2\nthe anchor line\nline 4\nline 5\nline 6\n",
        )
        .unwrap();

        let mut config = AppConfig::from_env();
        config.project_root = tmp.to_string_lossy().into_owned();

        let diagnosis = diagnose_not_found(
            "foo.rs",
            "the anchor line\nbut this part does not exist",
            &config,
        )
        .await;

        match diagnosis {
            RetryDiagnosis::AnchorLineFound {
                anchor_line_no,
                anchor_line_text,
                file_slice,
            } => {
                assert_eq!(anchor_line_no, 3);
                assert_eq!(anchor_line_text, "the anchor line");
                assert!(file_slice.text.contains("the anchor line"));
                assert!(file_slice.text.contains("line 6"));
            }
            other => panic!("expected AnchorLineFound, got {other:?}"),
        }

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn build_retry_directive_renders_anchor_found_hit() {
        let evidence = RetryEvidence {
            hits: vec![AnchorRetryHit {
                path: "src/config.rs".to_string(),
                edit_index: 0,
                edit_total: 1,
                failed_search: "openrouter_api_key: x,\n        }".to_string(),
                status: AnchorStatus::NotFound,
                diagnosis: RetryDiagnosis::AnchorLineFound {
                    anchor_line_no: 67,
                    anchor_line_text: "openrouter_api_key: x,".to_string(),
                    file_slice: FileExcerpt {
                        path: "src/config.rs".to_string(),
                        start_line: 62,
                        end_line: 122,
                        text: "(real file slice goes here)".to_string(),
                    },
                },
            }],
        };

        let directive = build_retry_directive("test original task", &evidence);
        assert!(directive.contains("Failure #1: src/config.rs"));
        assert!(directive.contains("status: [NOT FOUND in file]"));
        assert!(directive.contains("openrouter_api_key: x,"));
        assert!(directive.contains("lines 62-122"));
        assert!(directive.contains("real file slice goes here"));
        assert!(directive.contains("LAST existing line"));
    }

    #[test]
    fn build_retry_directive_renders_missing_anchor_hit() {
        let evidence = RetryEvidence {
            hits: vec![AnchorRetryHit {
                path: "src/foo.rs".to_string(),
                edit_index: 1,
                edit_total: 2,
                failed_search: "let totally_made_up = 1;".to_string(),
                status: AnchorStatus::NotFound,
                diagnosis: RetryDiagnosis::AnchorLineMissing,
            }],
        };
        let directive = build_retry_directive("test original task", &evidence);
        assert!(directive.contains("edit 2 of 2"));
        assert!(directive.contains("Not a single line of your SEARCH was found"));
    }

    #[test]
    fn build_retry_directive_renders_create_on_existing_hit() {
        let evidence = RetryEvidence {
            hits: vec![AnchorRetryHit {
                path: "src/config.rs".to_string(),
                edit_index: 0,
                edit_total: 1,
                failed_search: String::new(),
                status: AnchorStatus::NotFound,
                diagnosis: RetryDiagnosis::CreateOnExistingFile {
                    file_head: FileExcerpt {
                        path: "src/config.rs".to_string(),
                        start_line: 1,
                        end_line: 25,
                        text: "use std::env;\n\npub struct AppConfig {\n    pub model_name: String,\n}".to_string(),
                    },
                },
            }],
        };

        let directive = build_retry_directive("test original task", &evidence);
        assert!(directive.contains("BLOCKED: you used new=true but the file already exists"));
        assert!(directive.contains("first 25 line(s)"));
        assert!(directive.contains("pub struct AppConfig"));
        assert!(directive.contains("Do not emit `new=true` again"));
        // Should NOT include the "Your SEARCH text was" line when failed_search is empty
        assert!(!directive.contains("Your SEARCH text was:"));
    }

    #[tokio::test]
    async fn diagnose_create_on_existing_reads_file_head() {
        let tmp = std::env::temp_dir().join(format!(
            "rustopedia_retry_create_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&tmp).unwrap();
        let file = tmp.join("foo.rs");
        let mut body = String::new();
        for i in 1..=80 {
            body.push_str(&format!("line {i}\n"));
        }
        std::fs::write(&file, body).unwrap();

        let mut config = AppConfig::from_env();
        config.project_root = tmp.to_string_lossy().into_owned();

        let diagnosis = diagnose_create_on_existing("foo.rs", &config).await;
        match diagnosis {
            RetryDiagnosis::CreateOnExistingFile { file_head } => {
                assert_eq!(file_head.start_line, 1);
                assert_eq!(file_head.end_line, 60); // CONTEXT_LINES_AFTER
                assert!(file_head.text.starts_with("line 1\n"));
                assert!(file_head.text.contains("line 60"));
                assert!(!file_head.text.contains("line 61"));
            }
            other => panic!("expected CreateOnExistingFile, got {other:?}"),
        }

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn has_retryable_failures_includes_create_on_existing() {
        use crate::patch::{ParsedPatches, Patch, VerifiedPatches};

        let parsed = ParsedPatches {
            patches: vec![Patch::Create {
                path: "src/config.rs".to_string(),
                content: "x".to_string(),
            }],
            errors: vec![],
        };

        let tmp = std::env::temp_dir().join(format!(
            "rustopedia_retryable_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&tmp).unwrap();
        let file = tmp.join("src/config.rs");
        std::fs::create_dir_all(file.parent().unwrap()).unwrap();
        std::fs::write(&file, "already here").unwrap();

        let verified: VerifiedPatches = crate::patch::verify(&parsed, &tmp);
        assert!(has_retryable_failures(&verified));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn build_retry_directive_returns_empty_for_no_hits() {
        let evidence = RetryEvidence::default();
        assert!(build_retry_directive("test original task", &evidence).is_empty());
    }

    #[test]
    fn build_retry_directive_echoes_original_task_and_counters_refusal() {
        let evidence = RetryEvidence {
            hits: vec![AnchorRetryHit {
                path: "src/foo.rs".to_string(),
                edit_index: 0,
                edit_total: 1,
                failed_search: "x".to_string(),
                status: AnchorStatus::NotFound,
                diagnosis: RetryDiagnosis::AnchorLineMissing,
            }],
        };
        let directive = build_retry_directive("add field foo to Bar", &evidence);
        assert!(directive.contains("add field foo to Bar"));
        assert!(directive.contains("Task remains"));
        assert!(directive.contains("do not refuse"));
        assert!(directive.contains("1-2 sentences"));
        assert!(directive.contains("Minimal prose only"));
    }

    #[tokio::test]
    async fn diagnose_not_found_marks_anchor_missing_when_first_line_absent() {
        let tmp = std::env::temp_dir().join(format!(
            "rustopedia_retry_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&tmp).unwrap();
        let file = tmp.join("foo.rs");
        std::fs::write(&file, "only line\n").unwrap();

        let mut config = AppConfig::from_env();
        config.project_root = tmp.to_string_lossy().into_owned();

        let diagnosis =
            diagnose_not_found("foo.rs", "this line is not in the file", &config).await;

        assert!(matches!(diagnosis, RetryDiagnosis::AnchorLineMissing));

        let _ = std::fs::remove_dir_all(&tmp);
    }
}
