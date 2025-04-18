use std::io::{self};
fn main() {
    println!("Enter a number between 1 and 5 for status");

    let mut status = String::new();

    // status is mutated here, allowing us to trim it and parse it to integer
    // in the next line
    match io::stdin().read_line(&mut status) {
        Ok(stat) => stat,
        Err(_) => panic!("Failed to read line"),
    };

    let task_status: u32 = match status.trim().parse() {
        Ok(stat) => stat,
        Err(_) => panic!("Error parsing string!"),
    };

    let task_status = match TaskStatus::from(task_status) {
        Some(stats) => stats,
        None => panic!("Invalid task status"),
    };

    let task = Task::new(1, String::from("test task"), task_status, None);

    println!("{}", task.status.to_string())
}

#[derive(Debug)]
enum TaskStatus {
    PENDING,
    COMPLETED,
    InPROGRESS,
    CANCELLED,
}

impl TaskStatus {
    // overriding the to_string method
    // to return a string representation of the enum
    fn to_string(&self) -> String {
        match self {
            TaskStatus::PENDING => String::from("Pending"),
            TaskStatus::COMPLETED => String::from("Completed"),
            TaskStatus::InPROGRESS => String::from("In Progress"),
            TaskStatus::CANCELLED => String::from("Cancelled"),
        }
    }

    fn from(status: u32) -> Option<Self> {
        match status {
            1 => Some(TaskStatus::PENDING),
            2 => Some(TaskStatus::COMPLETED),
            3 => Some(TaskStatus::InPROGRESS),
            4 => Some(TaskStatus::CANCELLED),
            5 => Some(TaskStatus::PENDING),
            _ => None,
        }
    }
}

type TaskNote = Option<String>;

#[derive(Debug)]
struct Task {
    id: u32,
    name: String,
    status: TaskStatus,
    note: TaskNote,
}

trait TaskTrait {
    fn new(id: u32, name: String, status: TaskStatus, note: TaskNote) -> Task;
}

impl TaskTrait for Task {
    fn new(id: u32, name: String, status: TaskStatus, note: TaskNote) -> Task {
        Task {
            id,
            name,
            status,
            note,
        }
    }
}
