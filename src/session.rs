use crate::intents::RustIntent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionMode {
    Ask,
    Review,
    Edit,
}

impl SessionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ask => "ask",
            Self::Review => "review",
            Self::Edit => "edit",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "ask" => Some(Self::Ask),
            "critique" | "review" => Some(Self::Review),
            "edit" => Some(Self::Edit),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConversationTurn {
    pub query: String,
    pub response: String,
}

#[derive(Debug, Default)]
pub struct SessionState {
    history: Vec<ConversationTurn>,
    mode: Option<SessionMode>,
    pending_clarification: Option<PendingClarification>,
    last_subject: Option<SubjectAnchor>,
    last_memory_snapshot: Option<MemorySnapshot>,
}

#[derive(Debug, Clone)]
pub struct PendingClarification {
    pub intent: RustIntent,
    pub original_query: String,
}

#[derive(Debug, Clone)]
pub struct SubjectAnchor {
    pub mode: SessionMode,
    pub intent: RustIntent,
    pub query: String,
}

#[derive(Debug, Clone)]
pub struct MemorySnapshot {
    pub mode: SessionMode,
    pub intent: RustIntent,
    pub working_memory_items: usize,
    pub file_excerpts: usize,
    pub diff_items: usize,
    pub text_items: usize,
    pub session_summary_chars: usize,
    pub background_summary_chars: usize,
    pub item_summaries: Vec<String>,
}

impl SessionState {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            mode: Some(SessionMode::Ask),
            pending_clarification: None,
            last_subject: None,
            last_memory_snapshot: None,
        }
    }

    pub fn mode(&self) -> SessionMode {
        self.mode.unwrap_or(SessionMode::Ask)
    }

    pub fn set_mode(&mut self, mode: SessionMode) {
        self.mode = Some(mode);
        self.pending_clarification = None;
    }

    pub fn history(&self) -> &[ConversationTurn] {
        &self.history
    }

    pub fn push_turn(&mut self, query: String, response: String) {
        self.history.push(ConversationTurn { query, response });
    }

    pub fn set_pending_clarification(&mut self, intent: RustIntent, original_query: String) {
        self.pending_clarification = Some(PendingClarification {
            intent,
            original_query,
        });
    }

    pub fn take_pending_clarification(&mut self) -> Option<PendingClarification> {
        self.pending_clarification.take()
    }

    pub fn set_last_subject(&mut self, mode: SessionMode, intent: RustIntent, query: String) {
        self.last_subject = Some(SubjectAnchor {
            mode,
            intent,
            query,
        });
    }

    pub fn last_subject(&self) -> Option<&SubjectAnchor> {
        self.last_subject.as_ref()
    }

    pub fn set_last_memory_snapshot(&mut self, snapshot: MemorySnapshot) {
        self.last_memory_snapshot = Some(snapshot);
    }

    pub fn last_memory_snapshot(&self) -> Option<&MemorySnapshot> {
        self.last_memory_snapshot.as_ref()
    }
}

pub enum ParsedInput<'a> {
    Exit,
    Command(Command<'a>),
    Query(&'a str),
}

pub enum Command<'a> {
    Help,
    Status,
    Memory,
    Mode(SessionMode),
    Unknown(&'a str),
}

pub fn parse_input(input: &str) -> ParsedInput<'_> {
    let trimmed = input.trim();
    if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
        return ParsedInput::Exit;
    }

    if let Some(rest) = trimmed.strip_prefix("/mode ") {
        return match SessionMode::from_str(rest) {
            Some(mode) => ParsedInput::Command(Command::Mode(mode)),
            None => ParsedInput::Command(Command::Unknown(trimmed)),
        };
    }

    match trimmed {
        "/help" => ParsedInput::Command(Command::Help),
        "/status" => ParsedInput::Command(Command::Status),
        "/memory" => ParsedInput::Command(Command::Memory),
        _ if trimmed.starts_with('/') => ParsedInput::Command(Command::Unknown(trimmed)),
        _ => ParsedInput::Query(trimmed),
    }
}
