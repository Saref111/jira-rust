pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

pub struct Epic {
    name: String,
    description: String,
    status: Status,
    stories: Vec<u64>,
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

pub struct Story {
    id: u64,
    name: String,
    description: String,
    status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        let last_item_id = 0;
        Self {
            id: last_item_id + 1,
            name,
            description,
            status: Status::Open
        }
    }
}

pub struct DBState {
    pub last_item_id: u64,
    pub epics: Vec<Epic>,
    pub stories: Vec<Story>,
    // TODO: add fields (make sure the fields are public)
}
