use super::tasks_definitions::{Task, TaskStatus, TaskTrait};
use super::utils::{create_json_data, open_file, write_json_to_file};

use serde_json::Value;

use std::{
    fs::{self},
    io::{self, Read},
};

impl Task {
    pub fn new() -> Self {
        //  Create ID
        let id = match next_id() {
            Some(i) => i,
            _ => 0,
        };

        Task {
            id,
            description: String::new(),
            status: None,
        }
    }
}

impl TaskTrait for Task {
    fn update(&self, id: &u32, description: &Option<String>) -> Result<Task, io::Error> {
        // get the task by id
        let mut task = match Self::get(id) {
            Some(res) => res,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Cannot find task with id {}", id,),
                ))
            }
        };

        task.description = match description {
            Some(desc) => desc.to_string(), // call to string to take ownership of the string data.
            None => task.description,       // take the old description if none is provided
        };

        // write to the file using the new data and save it.
        let mut file = open_file()?;

        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;

        let mut json_data = create_json_data(&file_content)?;

        let tasks_array = json_data["tasks"]
            .as_array_mut()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "tasks is not an array"))?;

        for task_value in tasks_array.iter_mut() {
            // If .and_then() returns None, the condition will be false
            // since None == Some is false.
            if task_value.get("id").and_then(|v| v.as_u64()) == Some(task.id as u64) {
                // Update the task value with the new task data by dereferencing the task_value
                // this will make sure that it mutates the original value
                *task_value = serde_json::to_value(&task).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Failed to serialize task: {}", e),
                    )
                })?;
                break;
            }
        }

        write_json_to_file(&mut file, &json_data)?;

        // return the task
        Ok(task)
    }

    fn get(id: &u32) -> Option<Task> {
        // get all tasks
        let tasks = match get_tasks() {
            Ok(value) => value,
            Err(_) => return None,
        };

        // return when there's no task
        if tasks.is_empty() {
            return None;
        }

        // loop through and find the task with the id
        for task_value in tasks {
            if let Ok(task) = serde_json::from_value::<Task>(task_value) {
                if task.id == *id {
                    return Some(task);
                }
            }
        }

        None
    }

    fn create(&self) -> Result<&'static str, io::Error> {
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

        let mut json_data = match create_json_data(&file_content) {
            Ok(value) => value,
            Err(e) => return Err(e),
        };

        let new_task = serde_json::to_value(&self)?;

        json_data["tasks"]
            .as_array_mut()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "tasks is not an array"))?
            .push(new_task);

        write_json_to_file(&mut task_file, &json_data)?;

        Ok("Task created successfully")
    }

    fn list(&self, status: Option<&TaskStatus>) -> Result<Vec<Value>, io::Error> {
        match status {
            Some(status) => sort_by_status(status), // only run with status if there is status
            None => get_tasks(),
        }
    }

    fn delete(&self, id: &u32) -> Result<&'static str, io::Error> {
        let mut file = open_file()?;

        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;

        let mut json_data = create_json_data(&file_content)?;

        let tasks_array = json_data["tasks"]
            .as_array_mut()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "tasks is not an array"))?;

        // find the index of the task to delete
        let index = match tasks_array
            .iter()
            .position(|value| value.get("id").and_then(|v| v.as_u64()) == Some(*id as u64))
        {
            Some(index) => index,
            _ => return Err(io::Error::new(io::ErrorKind::NotFound, "Task not found")),
        };

        tasks_array.remove(index);

        let json_string = serde_json::to_string_pretty(&tasks_array)?;

        write_json_to_file(&mut file, &json_data)?;

        println!("Tasks array after deletion: {}", json_string);

        Ok("Task deleted successfully")
    }
    // fn delete(&self, task: &mut Task) -> Task {}
}

fn get_tasks() -> Result<Vec<Value>, io::Error> {
    let mut task_file = open_file()?;

    let mut file_content = String::new();
    task_file.read_to_string(&mut file_content)?;

    let tasks: Value = serde_json::from_str(&file_content)?;

    let tasks: Vec<Value> = tasks["tasks"]
        .as_array()
        .map(|arr| arr.to_vec())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "tasks is not an array"))?;

    Ok(tasks)
}

fn next_id() -> Option<u32> {
    let tasks = match get_tasks() {
        Err(_) => return None,
        Ok(res) => res,
    };

    let ids = match tasks
        .iter()
        .map(|task| match task.get("id") {
            Some(id) => match id.as_u64() {
                Some(id) => id,
                _ => 0,
            },
            _ => 0,
        })
        .max()
    {
        Some(id) => id,
        _ => 0,
    };

    Some(ids as u32 + 1)
}

fn sort_by_status(status: &TaskStatus) -> Result<Vec<Value>, io::Error> {
    let tasks = get_tasks()?
        .into_iter()
        .filter(|task| task["status"] == status.to_string().to_lowercase())
        .collect::<Vec<Value>>();

    Ok(tasks)
}
