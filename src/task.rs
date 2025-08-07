use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            status: TaskStatus::Todo,
        }
    }
}
