

/// Unique record identifier (auto-increment u64, stored as big-endian in B+tree)
pub type RecordId = u64;

/// Unix seconds timestamp
pub type Timestamp = i64;

/// Currency amount in minor units (cents)
pub type CurrencyAmount = i64;

/// A full name (first + last)
#[derive(Debug, Clone, PartialEq)]
pub struct FullName {
    pub first: String,
    pub last: String,
}

impl FullName {
    pub fn new(first: &str, last: &str) -> Self {
        Self { first: first.into(), last: last.into() }
    }
    pub fn display(&self) -> String {
        format!("{} {}", self.first, self.last)
    }
}

/// Email address with label
#[derive(Debug, Clone, PartialEq)]
pub struct Email {
    pub address: String,
    pub label: String, // "work", "personal", etc.
}

/// Phone number with label
#[derive(Debug, Clone, PartialEq)]
pub struct Phone {
    pub number: String,
    pub label: String,
}

/// Link (URL) with label
#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    pub url: String,
    pub label: String,
}

/// Address
#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
}

/// An entity's stage in a pipeline (for opportunities)
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineStage {
    New,
    Screening,
    Meeting,
    Proposal,
    Negotiation,
    Won,
    Lost,
}

impl PipelineStage {
    pub fn all() -> &'static [Self] {
        &[Self::New, Self::Screening, Self::Meeting, Self::Proposal, Self::Negotiation, Self::Won, Self::Lost]
    }
    pub fn label(&self) -> &str {
        match self {
            Self::New => "New",
            Self::Screening => "Screening",
            Self::Meeting => "Meeting",
            Self::Proposal => "Proposal",
            Self::Negotiation => "Negotiation",
            Self::Won => "Won",
            Self::Lost => "Lost",
        }
    }
}

/// Task status
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Cancelled,
}

impl TaskStatus {
    pub fn all() -> &'static [Self] {
        &[Self::Todo, Self::InProgress, Self::Done, Self::Cancelled]
    }
    pub fn label(&self) -> &str {
        match self {
            Self::Todo => "Todo",
            Self::InProgress => "In Progress",
            Self::Done => "Done",
            Self::Cancelled => "Cancelled",
        }
    }
}

/// Polymorphic entity reference
#[derive(Debug, Clone, PartialEq)]
pub struct EntityRef {
    pub entity_type: EntityType,
    pub record_id: RecordId,
    pub display_name: String,
}

/// Entity type identifier
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum EntityType {
    Person,
    Company,
    Opportunity,
    Task,
    Note,
    User,
}

impl EntityType {
    pub fn plural(&self) -> &str {
        match self {
            Self::Person => "people",
            Self::Company => "companies",
            Self::Opportunity => "opportunities",
            Self::Task => "tasks",
            Self::Note => "notes",
            Self::User => "users",
        }
    }
    pub fn singular(&self) -> &str {
        match self {
            Self::Person => "person",
            Self::Company => "company",
            Self::Opportunity => "opportunity",
            Self::Task => "task",
            Self::Note => "note",
            Self::User => "user",
        }
    }
    pub fn label_plural(&self) -> &str {
        match self {
            Self::Person => "People",
            Self::Company => "Companies",
            Self::Opportunity => "Opportunities",
            Self::Task => "Tasks",
            Self::Note => "Notes",
            Self::User => "Users",
        }
    }
}
