use std::{collections::HashMap, fmt::Display};

use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl From<String> for Status {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Open" => Status::Open,
            "InProgress" => Status::InProgress,
            "Resolved" => Status::Resolved,
            "Closed" => Status::Closed,
            _ => Status::Open,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Open => write!(f, "OPEN"),
            Self::Closed => write!(f, "CLOSED"),
            Self::InProgress => write!(f, "IN PROGRESS"),
            Self::Resolved => write!(f, "RESOLVED")
        }
    }
}
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Eq, PartialOrd, Ord)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
            stories: Vec::new(),
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Eq, PartialOrd, Ord)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToEpicDetail { epic_id: u32 },
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory { epic_id: u32 },
    UpdateStoryStatus { story_id: u32 },
    DeleteStory { epic_id: u32, story_id: u32 },
    Exit,
}
