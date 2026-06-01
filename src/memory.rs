use crate::intents::RustIntent;
use crate::session::SessionMode;

#[derive(Debug, Clone)]
pub struct MemoryState {
    pub current_task: CurrentTask,
    pub working_memory: Vec<WorkingMemoryItem>,
    pub session_summary: String,
    pub background_summary: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CurrentTask {
    pub mode: SessionMode,
    pub intent: RustIntent,
    pub query: String,
}

#[derive(Debug, Clone)]
pub enum WorkingMemoryItem {
    Text { label: String, content: String },
    FileExcerpt(FileExcerpt),
    DiffSummary(DiffSummary),
}

#[derive(Debug, Clone)]
pub struct FileExcerpt {
    pub path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct DiffSummary {
    pub changed_files: Vec<String>,
    pub summary: String,
}

impl MemoryState {
    pub fn working_memory_text(&self) -> String {
        if self.working_memory.is_empty() {
            return "No fresh working-memory evidence was gathered for this turn.".to_string();
        }

        self.working_memory
            .iter()
            .map(WorkingMemoryItem::render)
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    pub fn background_summary_text(&self) -> &str {
        self.background_summary
            .as_deref()
            .unwrap_or("No background summary.")
    }
}

impl WorkingMemoryItem {
    pub fn char_count(&self) -> usize {
        self.render().chars().count()
    }

    pub fn summary_label(&self) -> String {
        match self {
            Self::Text { label, .. } => format!("text:{label}"),
            Self::FileExcerpt(excerpt) => {
                format!(
                    "file:{}:{}-{}",
                    excerpt.path, excerpt.start_line, excerpt.end_line
                )
            }
            Self::DiffSummary(diff) => format!("diff:{} files", diff.changed_files.len()),
        }
    }

    pub fn render(&self) -> String {
        match self {
            Self::Text { label, content } => format!("{label}:\n{}", content.trim()),
            Self::FileExcerpt(excerpt) => format!(
                "From {} (lines {}-{}):\n```rust\n{}\n```",
                excerpt.path, excerpt.start_line, excerpt.end_line, excerpt.text
            ),
            Self::DiffSummary(diff) => format!("Workspace Diff:\n{}", diff.summary.trim()),
        }
    }
}
