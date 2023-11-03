use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
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
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
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

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
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
