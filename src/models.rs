use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
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
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<i32>,
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

#[derive(PartialEq, Debug, Serialize, Deserialize)]
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

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct DBState {
    pub last_item_id: i32,
    pub epics: HashMap<i32, Epic>,
    pub stories: HashMap<i32, Story>,
}
