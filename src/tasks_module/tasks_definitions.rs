use serde_json::Value;
use std::io::{self};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum TaskStatus {
    PENDING,
    COMPLETED,
    InPROGRESS,
}

impl TaskStatus {
    // overriding the to_string method
    // to return a string representation of the enum
    pub fn to_string(&self) -> String {
        match self {
            TaskStatus::PENDING => String::from("Pending"),
            TaskStatus::COMPLETED => String::from("Completed"),
            TaskStatus::InPROGRESS => String::from("In Progress"),
        }
    }

    pub fn from(status: &str) -> Option<Self> {
        match status {
            "todo" | "pending" => Some(TaskStatus::PENDING),
            "done" | "completed" => Some(TaskStatus::COMPLETED),
            "in-progress" | "in progress" | "in_progress" => Some(TaskStatus::InPROGRESS),
            _ => None,
        }
    }
}

pub trait TaskTrait {
    fn update(&self, id: &u32, description: &Option<String>) -> Result<Task, io::Error>;
    // fn delete(&self, task: &mut Task) -> Task;
    fn get(id: &u32) -> Option<Task>;
    fn create(&self) -> Result<&'static str, io::Error>;
    fn delete(&self, id: &u32) -> Result<&'static str, io::Error>;
    fn list(&self) -> Result<Vec<Value>, io::Error>;
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: Option<TaskStatus>,
}
