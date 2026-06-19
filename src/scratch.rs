//! Scratch git worktree used to apply candidate patches and validate them
//! (e.g. `cargo check`) without touching the user's real working tree.
//!
//! The overlay is created from the project's current `HEAD`, then any
//! uncommitted tracked changes and untracked-but-not-ignored files are
//! mirrored in so that the validation runs see the same code the user is
//! currently editing — not just whatever was last committed.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, anyhow, bail};

use crate::patch::{VerifiedPatch, VerifiedPatches};

/// A temporary git worktree the runtime can apply patches into and run
/// cargo against. Removed automatically on drop.
pub struct ScratchOverlay {
    project_root: PathBuf,
    overlay_root: PathBuf,
    cleaned_up: bool,
}

impl ScratchOverlay {
    /// Create a new scratch worktree mirroring the current working tree of
    /// `project_root`. Returns an error if the project is not a git
    /// repository, has no `HEAD`, or if the `git worktree add` invocation
    /// fails.
    pub fn create(project_root: &Path) -> Result<Self> {
        let overlay_root = unique_overlay_dir();

        let status = Command::new("git")
            .arg("-C")
            .arg(project_root)
            .arg("worktree")
            .arg("add")
            .arg("--detach")
            .arg(&overlay_root)
            .arg("HEAD")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .context("failed to spawn git worktree add")?;

        if !status.status.success() {
            let stderr = String::from_utf8_lossy(&status.stderr).to_string();
            bail!("git worktree add failed: {}", stderr.trim());
        }

        let overlay = Self {
            project_root: project_root.to_path_buf(),
            overlay_root,
            cleaned_up: false,
        };

        if let Err(err) = overlay.mirror_uncommitted_tracked() {
            overlay.try_cleanup_sync();
            return Err(err);
        }
        if let Err(err) = overlay.mirror_untracked() {
            overlay.try_cleanup_sync();
            return Err(err);
        }

        Ok(overlay)
    }

    pub fn root(&self) -> &Path {
        &self.overlay_root
    }

    /// Apply verified patches into the overlay. Only patches whose anchors
    /// were already verified clean against the real tree are applied —
    /// callers are expected to have checked that `patches_applicable` covers
    /// every edit first. Anything still anchored as non-applicable here is
    /// silently skipped (the validation retry only fires on otherwise-clean
    /// runs).
    pub fn apply(&self, verified: &VerifiedPatches) -> Result<()> {
        for patch in &verified.patches {
            match patch {
                VerifiedPatch::Modify { path, edits } => {
                    let full_path = self.overlay_root.join(path);
                    let mut content = fs::read_to_string(&full_path)
                        .with_context(|| format!("failed to read overlay file {path}"))?;
                    for edit in edits {
                        if !edit.status.is_applicable() {
                            continue;
                        }
                        let matches = content.matches(&edit.edit.search).count();
                        if matches == 0 {
                            // Tolerate duplicates: if the SEARCH is gone but
                            // the REPLACE is already present, an earlier edit
                            // in this same patch set already produced the
                            // intended state. Skip silently.
                            if content.contains(&edit.edit.replace) {
                                continue;
                            }
                            return Err(anyhow!(
                                "overlay anchor missing in {path}: SEARCH text not found and REPLACE not present"
                            ));
                        }
                        if matches > 1 {
                            return Err(anyhow!(
                                "overlay anchor ambiguous in {path}: {matches} matches"
                            ));
                        }
                        content = content.replacen(&edit.edit.search, &edit.edit.replace, 1);
                    }
                    fs::write(&full_path, content)
                        .with_context(|| format!("failed to write overlay file {path}"))?;
                }
                VerifiedPatch::Create {
                    path,
                    content,
                    file_already_exists,
                } => {
                    if *file_already_exists {
                        continue;
                    }
                    let full_path = self.overlay_root.join(path);
                    if let Some(parent) = full_path.parent() {
                        fs::create_dir_all(parent).with_context(|| {
                            format!("failed to create overlay parent dirs for {path}")
                        })?;
                    }
                    fs::write(&full_path, content)
                        .with_context(|| format!("failed to create overlay file {path}"))?;
                }
                VerifiedPatch::Symbolic { path, edits } => {
                    let full_path = self.overlay_root.join(path);
                    let mut content = fs::read_to_string(&full_path)
                        .with_context(|| format!("failed to read overlay file {path}"))?;

                    // Apply in descending start-line order so earlier splices don't
                    // invalidate the byte offsets of later (earlier-in-file) edits.
                    let mut applicable: Vec<&crate::patch::VerifiedSymbolicEdit> = edits
                        .iter()
                        .filter(|e| e.status.is_applicable() && e.span.is_some())
                        .collect();
                    applicable.sort_by_key(|e| std::cmp::Reverse(e.span.unwrap().0));

                    for edit in applicable {
                        let (start_line, end_line) = edit.span.unwrap();
                        let (start_byte, end_byte) =
                            line_range_to_byte_range(&content, start_line, end_line);
                        let body = edit.edit.body.trim_end_matches('\n');
                        match edit.edit.op {
                            crate::patch::SymbolicOp::Replace => {
                                content.replace_range(start_byte..end_byte, body);
                            }
                            crate::patch::SymbolicOp::InsertAfter => {
                                if content.contains(body.trim()) {
                                    continue; // idempotent: already inserted
                                }
                                content.insert_str(end_byte, &format!("\n\n{body}"));
                            }
                            crate::patch::SymbolicOp::InsertBefore => {
                                if content.contains(body.trim()) {
                                    continue;
                                }
                                content.insert_str(start_byte, &format!("{body}\n\n"));
                            }
                        }
                    }

                    fs::write(&full_path, content)
                        .with_context(|| format!("failed to write overlay file {path}"))?;
                }
            }
        }
        Ok(())
    }

    fn mirror_uncommitted_tracked(&self) -> Result<()> {
        let diff = Command::new("git")
            .arg("-C")
            .arg(&self.project_root)
            .arg("diff")
            .arg("HEAD")
            .output()
            .context("failed to spawn git diff HEAD")?;

        if !diff.status.success() {
            bail!(
                "git diff HEAD failed: {}",
                String::from_utf8_lossy(&diff.stderr).trim()
            );
        }

        if diff.stdout.is_empty() {
            return Ok(());
        }

        let mut child = Command::new("git")
            .arg("-C")
            .arg(&self.overlay_root)
            .arg("apply")
            .arg("--whitespace=nowarn")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("failed to spawn git apply in overlay")?;

        child
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow!("git apply stdin unavailable"))?
            .write_all(&diff.stdout)
            .context("failed to pipe diff into git apply")?;

        let out = child
            .wait_with_output()
            .context("failed to wait for git apply")?;
        if !out.status.success() {
            bail!(
                "git apply (mirror uncommitted) failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }

        Ok(())
    }

    fn mirror_untracked(&self) -> Result<()> {
        let listing = Command::new("git")
            .arg("-C")
            .arg(&self.project_root)
            .arg("ls-files")
            .arg("--others")
            .arg("--exclude-standard")
            .output()
            .context("failed to spawn git ls-files --others")?;
        if !listing.status.success() {
            bail!(
                "git ls-files --others failed: {}",
                String::from_utf8_lossy(&listing.stderr).trim()
            );
        }

        let listing = String::from_utf8_lossy(&listing.stdout);
        for relative in listing.lines() {
            let relative = relative.trim();
            if relative.is_empty() {
                continue;
            }
            let src = self.project_root.join(relative);
            if !src.is_file() {
                continue; // skip directories or vanished entries
            }
            let dst = self.overlay_root.join(relative);
            if let Some(parent) = dst.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("failed to create overlay parent dirs for {relative}")
                })?;
            }
            fs::copy(&src, &dst)
                .with_context(|| format!("failed to copy untracked file {relative}"))?;
        }

        Ok(())
    }

    fn try_cleanup_sync(&self) {
        let _ = Command::new("git")
            .arg("-C")
            .arg(&self.project_root)
            .arg("worktree")
            .arg("remove")
            .arg("--force")
            .arg(&self.overlay_root)
            .output();
        let _ = fs::remove_dir_all(&self.overlay_root);
    }
}

impl Drop for ScratchOverlay {
    fn drop(&mut self) {
        if self.cleaned_up {
            return;
        }
        self.try_cleanup_sync();
        self.cleaned_up = true;
    }
}

/// Map a 1-based inclusive line range to a byte range `[start, end)` where `start` is the
/// first byte of `start_line` and `end` is the byte index of the newline terminating
/// `end_line` (or EOF if it has no trailing newline). Replacing this range swaps the
/// item's lines while leaving the terminating newline intact.
fn line_range_to_byte_range(content: &str, start_line: usize, end_line: usize) -> (usize, usize) {
    let mut start_byte = if start_line <= 1 { 0 } else { content.len() };
    let mut end_byte = content.len();
    let mut current = 1usize;
    for (i, ch) in content.char_indices() {
        if ch == '\n' {
            if current == end_line {
                end_byte = i;
            }
            current += 1;
            if current == start_line {
                start_byte = i + 1;
            }
        }
    }
    (start_byte, end_byte)
}

fn unique_overlay_dir() -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    std::env::temp_dir().join(format!(
        "rustopedia_overlay_{}_{}",
        std::process::id(),
        nanos
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patch::{
        ItemSelector, ParsedPatches, Patch, SearchReplaceEdit, SymbolicEdit, SymbolicOp,
        VerifiedPatches, verify as verify_patches,
    };
    use std::process::Command as SyncCommand;

    fn init_git_repo(root: &Path) {
        SyncCommand::new("git")
            .arg("-C")
            .arg(root)
            .args(["init", "--quiet"])
            .status()
            .unwrap();
        // Ensure committer identity exists for this throwaway repo.
        for (k, v) in [
            ("user.email", "test@example.com"),
            ("user.name", "Test"),
            ("commit.gpgsign", "false"),
        ] {
            SyncCommand::new("git")
                .arg("-C")
                .arg(root)
                .args(["config", k, v])
                .status()
                .unwrap();
        }
    }

    fn commit_all(root: &Path, message: &str) {
        SyncCommand::new("git")
            .arg("-C")
            .arg(root)
            .args(["add", "-A"])
            .status()
            .unwrap();
        SyncCommand::new("git")
            .arg("-C")
            .arg(root)
            .args(["commit", "-m", message, "--quiet"])
            .status()
            .unwrap();
    }

    fn tmpdir(label: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let p = std::env::temp_dir().join(format!(
            "rustopedia_scratch_{label}_{}_{}",
            std::process::id(),
            nanos
        ));
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn overlay_mirrors_uncommitted_changes_and_applies_patch() {
        let root = tmpdir("apply");
        init_git_repo(&root);
        fs::write(root.join("a.txt"), "hello\nworld\n").unwrap();
        commit_all(&root, "init");

        // uncommitted modification
        fs::write(root.join("a.txt"), "hello\nbrave\nworld\n").unwrap();
        // untracked file
        fs::write(root.join("b.txt"), "untracked\n").unwrap();

        let overlay = ScratchOverlay::create(&root).expect("overlay create");
        assert!(overlay.root().join("a.txt").exists());
        let a_contents = fs::read_to_string(overlay.root().join("a.txt")).unwrap();
        assert!(
            a_contents.contains("brave"),
            "overlay must reflect uncommitted edit, got {a_contents:?}"
        );
        let b_contents = fs::read_to_string(overlay.root().join("b.txt")).unwrap();
        assert_eq!(b_contents, "untracked\n");

        // Apply a patch that replaces "brave" with "bold" — confirm only
        // the overlay file changes, real tree is untouched.
        let parsed = ParsedPatches {
            patches: vec![Patch::Modify {
                path: "a.txt".to_string(),
                edits: vec![SearchReplaceEdit {
                    search: "brave".to_string(),
                    replace: "bold".to_string(),
                }],
            }],
            errors: vec![],
        };
        let verified: VerifiedPatches = verify_patches(&parsed, overlay.root());
        overlay.apply(&verified).expect("apply");

        let overlay_a = fs::read_to_string(overlay.root().join("a.txt")).unwrap();
        assert!(overlay_a.contains("bold"));
        let real_a = fs::read_to_string(root.join("a.txt")).unwrap();
        assert!(
            real_a.contains("brave"),
            "real tree must be untouched, got {real_a:?}"
        );

        drop(overlay);
        let _ = fs::remove_dir_all(&root);
    }

    fn symbolic_overlay(root: &Path, file: &str, contents: &str) -> ScratchOverlay {
        init_git_repo(root);
        let full = root.join(file);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&full, contents).unwrap();
        commit_all(root, "init");
        ScratchOverlay::create(root).expect("overlay create")
    }

    #[test]
    fn symbolic_replace_swaps_item_span() {
        let root = tmpdir("sym_replace");
        let overlay = symbolic_overlay(
            &root,
            "src/llm.rs",
            "struct Foo {\n    x: i32,\n}\n\nfn other() {}\n",
        );
        let parsed = ParsedPatches {
            patches: vec![Patch::Symbolic {
                path: "src/llm.rs".into(),
                edits: vec![SymbolicEdit {
                    op: SymbolicOp::Replace,
                    selector: ItemSelector::Struct("Foo".into()),
                    body: "struct Foo {\n    x: i64,\n}".into(),
                }],
            }],
            errors: vec![],
        };
        let verified = verify_patches(&parsed, overlay.root());
        overlay.apply(&verified).expect("apply");
        let out = fs::read_to_string(overlay.root().join("src/llm.rs")).unwrap();
        assert!(out.contains("x: i64,"), "got: {out:?}");
        assert!(!out.contains("x: i32,"));
        assert!(out.contains("fn other() {}"), "neighbour untouched: {out:?}");
        drop(overlay);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn symbolic_after_inserts_and_is_idempotent() {
        let root = tmpdir("sym_after");
        let overlay = symbolic_overlay(&root, "src/llm.rs", "struct Foo {\n    x: i32,\n}\n");
        let parsed = ParsedPatches {
            patches: vec![Patch::Symbolic {
                path: "src/llm.rs".into(),
                edits: vec![SymbolicEdit {
                    op: SymbolicOp::InsertAfter,
                    selector: ItemSelector::Struct("Foo".into()),
                    body: "impl Foo { fn n() {} }".into(),
                }],
            }],
            errors: vec![],
        };
        let verified = verify_patches(&parsed, overlay.root());
        overlay.apply(&verified).expect("apply");
        overlay.apply(&verified).expect("apply again"); // idempotent
        let out = fs::read_to_string(overlay.root().join("src/llm.rs")).unwrap();
        assert_eq!(out.matches("impl Foo { fn n() {} }").count(), 1, "got: {out:?}");
        // inserted after the struct's closing brace
        let struct_end = out.find('}').unwrap();
        let impl_pos = out.find("impl Foo").unwrap();
        assert!(impl_pos > struct_end);
        drop(overlay);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn overlay_create_fails_outside_git_repo() {
        let root = tmpdir("notgit");
        let result = ScratchOverlay::create(&root);
        assert!(result.is_err());
        let _ = fs::remove_dir_all(&root);
    }
}
