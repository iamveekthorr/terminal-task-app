use serde_json::{json, Value};
use std::{
    fs::{self, OpenOptions},
    io::{self, Read, Seek, Write},
};

fn main() {
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

    println!("Enter task name: ");
    let mut task_description = String::new();

    match io::stdin().read_line(&mut task_description) {
        Ok(description) => description,
        Err(_) => panic!("Failed to read line"),
    };

    let task = Task::new(1, task_description, task_status);

    match task.create() {
        Ok(res) => println!("{}", res),
        Err(e) => {
            eprintln!("This is from main...: {}", e);
            panic!("Error creating task: {}", e);
        }
    };
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Task {
    id: u32,
    description: String,
    status: TaskStatus,
}

trait TaskTrait {
    fn update(&self, id: u32, description: Option<String>) -> Task;
    // fn delete(&self, task: &mut Task) -> Task;
    fn get(&self) -> Option<Task>;
    fn create(&self) -> Result<String, io::Error>;
}

fn open_file() -> Result<fs::File, io::Error> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("tasks.json");

    match file {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    }
}

fn create_json_data(file_content: String) -> Result<Value, Result<String, io::Error>> {
    let json_data: Value = if file_content.trim().is_empty() {
        let json_string = json!({
            "tasks": []
        });

        match serde_json::from_value(json_string) {
            Ok(data) => data,
            Err(e) => {
                return Err(Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Error parsing JSON: {}", e),
                )));
            }
        }
    } else {
        match serde_json::from_str(&file_content) {
            Ok(data) => data,
            Err(e) => {
                return Err(Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Error parsing JSON: {}", e),
                )));
            }
        }
    };
    Ok(json_data)
}

impl TaskTrait for Task {
    fn update(&self, id: u32, description: Option<String>) -> Task {
        let mut task = Task {
            id,
            description: String::new(),
            status: TaskStatus::PENDING,
        };

        if let Some(desc) = description {
            task.description = desc;
        }

        task
    }

    fn get(&self) -> Option<Task> {
        let mut task_file = match open_file() {
            Ok(res) => res,
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => panic!("File not found!"),
                io::ErrorKind::PermissionDenied => panic!("Permission denied!"),
                _ => panic!("Error opening file: {}", e),
            },
        };

        let mut file_content = String::new();

        let tasks_results = match task_file.read_to_string(&mut file_content) {
            Ok(r) => r,
            Err(e) => panic!("Error reading file: {}", e),
        };

        let tasks: Vec<Task> = match serde_json::from_str::<Vec<Task>>(&tasks_results.to_string()) {
            Ok(tasks) => tasks,
            Err(e) => panic!("Error parsing JSON: {}", e),
        };

        match tasks.into_iter().find(|task| task.id == self.id) {
            Some(task) => Some(task),
            None => panic!("Task not found!"),
        }
    }

    fn create(&self) -> Result<String, io::Error> {
        let file: Result<fs::File, io::Error> = open_file();

        let mut task_file = match file {
            Ok(res) => res,
            Err(e) => return Err(e), // return the error if it is not a file not found error
        };

        let mut file_content = String::new();

        match task_file.read_to_string(&mut file_content) {
            Ok(res) => res,
            Err(e) => {
                return Err(e);
            }
        };

        let mut json_data = match create_json_data(file_content) {
            Ok(value) => value,
            Err(value) => return value,
        };

        let new_task = match serde_json::to_value(self) {
            Ok(value) => value,
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Error parsing JSON: {}", e),
                ));
            }
        };

        match json_data["tasks"].as_array_mut() {
            Some(tasks) => tasks.push(new_task),
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "tasks is not an array",
                ));
            }
        };

        let data = match serde_json::to_string_pretty(&json_data) {
            Ok(json) => json,
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Error parsing JSON: {}", e),
                ));
            }
        };

        // Reset file
        // use optional to exit on error
        task_file.seek(io::SeekFrom::Start(0))?;
        task_file.set_len(0)?;
        task_file.flush()?;

        // write back to file
        match task_file.write_all(data.as_bytes()) {
            Ok(_) => Ok(String::from("File Created Successfully")),
            Err(e) => return Err(e),
        }
    }

    // fn delete(&self, task: &mut Task) -> Task {}
}

impl Task {
    fn new(id: u32, description: String, status: TaskStatus) -> Self {
        Task {
            id,
            description,
            status,
        }
    }
}
