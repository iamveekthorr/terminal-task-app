use serde_json::Value;
use std::io::{self};

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Done,
    #[serde(rename = "in-progress")]
    InPROGRESS,
}

impl TaskStatus {
    // overriding the to_string method
    // to return a string representation of the enum
    pub fn to_string(&self) -> String {
        match self {
            TaskStatus::Pending => String::from("pending"),
            TaskStatus::Done => String::from("done"),
            TaskStatus::InPROGRESS => String::from("in-progress"),
        }
    }

    pub fn from(status: &str) -> Option<Self> {
        match status.to_lowercase().as_str() {
            "todo" | "pending" => Some(TaskStatus::Pending),
            "done" | "completed" => Some(TaskStatus::Done),
            "in-progress" | "in progress" | "in_progress" => Some(TaskStatus::InPROGRESS),
            _ => None,
        }
    }
}

pub trait TaskTrait {
    fn update(&self, id: &u32, description: &Option<String>) -> Result<Task, io::Error>;
    fn get(id: &u32) -> Option<Task>;
    fn create(&self) -> Result<&'static str, io::Error>;
    fn delete(&self, id: &u32) -> Result<&'static str, io::Error>;
    fn list(&self, status: Option<&TaskStatus>) -> Result<Vec<Value>, io::Error>;
    fn update_task_as_done(&self, id: &u32) -> Result<&'static str, io::Error>;
    fn update_task_as_in_progress(&self, id: &u32) -> Result<&'static str, io::Error>;
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: Option<TaskStatus>,
}
