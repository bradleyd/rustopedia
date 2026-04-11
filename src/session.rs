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
}

impl SessionState {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            mode: Some(SessionMode::Ask),
        }
    }

    pub fn mode(&self) -> SessionMode {
        self.mode.unwrap_or(SessionMode::Ask)
    }

    pub fn set_mode(&mut self, mode: SessionMode) {
        self.mode = Some(mode);
    }

    pub fn history(&self) -> &[ConversationTurn] {
        &self.history
    }

    pub fn push_turn(&mut self, query: String, response: String) {
        self.history.push(ConversationTurn { query, response });
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
        _ => ParsedInput::Query(trimmed),
    }
}
