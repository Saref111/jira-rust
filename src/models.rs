use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(PartialEq, Debug)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u64>,
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

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub struct DBState {
    pub last_item_id: i32,
    pub epics: HashMap<i32, Epic>,
    pub stories: HashMap<i32, Story>,
}
