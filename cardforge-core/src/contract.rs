//! Open CardForge — card contract v2 types.
//!
//! Apache-2.0. This crate must never contain AnkiTov-private hosts
//! (100.116.49.86, 100.111.137.43, 127.0.0.1:9200, 127.0.0.1:8787) or private
//! paths (.goose/). Those live in AnkiTov's private RuntimeProfile, not here.

pub const RULES_VERSION: &str = "1";

/// Which gate layers run. A mode changes WHICH layers run; it can never disable a
/// core structural check (see specs/qa/bar1-rules.yaml).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateMode {
    Curriculum,
    Personal,
}

impl GateMode {
    pub fn layers(&self) -> &'static [&'static str] {
        match self {
            GateMode::Curriculum => &["core", "institutional"],
            GateMode::Personal => &["core"],
        }
    }
}

/// How a blueprint was sourced. Drives which institutional checks apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceType {
    Standard,
    Wordlist,
    PersonalPrompt,
    UserNotes,
    ImportedDeck,
}

impl SourceType {
    /// Citation grounding runs only for standards and word lists.
    pub fn citation_grounding_applies(&self) -> bool {
        matches!(self, SourceType::Standard | SourceType::Wordlist)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteType {
    Basic,
    Cloze,
    TypeIn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Foundation,
    Core,
    Stretch,
}

/// The immutable skeleton Python/Rust supplies before any LLM call.
#[derive(Debug, Clone)]
pub struct ItemSpec {
    pub index: usize,
    pub concept_id: String,
    pub unit: String,
    pub grades: Vec<String>,
    pub term: String,
    pub assigned_note_type: NoteType,
    pub assigned_difficulty: Difficulty,
    pub source_citation: String,
    pub source_definition: Option<String>,
    pub curriculum_terms: Vec<String>,
    /// Every concept in a Blueprint is mandatory — no reserve-swap.
    pub mandatory: bool,
}

/// The LLM writes ONLY these fields.
#[derive(Debug, Clone)]
pub struct CardDraft {
    pub index: usize,
    pub term: String,
    pub definition: String,
    pub example: String,
    pub cloze_sent: Option<String>,
    pub notes: Option<String>,
}

pub fn concept_id_is_valid(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() && c.is_ascii_alphabetic() => {}
        _ => return false,
    }
    s[1..]
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

pub fn unit_is_valid(s: &str) -> bool {
    s.len() == 2 && s.chars().all(|c| c.is_ascii_digit())
}
