use super::tasks_definitions::{Task, TaskTrait};
use super::utils::{create_json_data, open_file, reset_file};
use serde_json::Value;
use std::{
    fs::{self},
    io::{self, Read, Write},
};

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Task {
            id,
            description,
            status: None,
        }
    }
}

impl TaskTrait for Task {
    fn update(id: &u32, description: Option<String>) -> Result<Task, io::Error> {
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
            Some(desc) => desc,
            None => task.description, // take the old description if none is provided
        };

        task.status = match task.status {
            Some(status) => Some(status),
            None => None,
        };

        Ok(task)
    }

    fn get(id: &u32) -> Option<Task> {
        let mut task_file = match open_file() {
            Ok(res) => res,
            _ => return None,
        };

        let mut file_content = String::new();

        match task_file.read_to_string(&mut file_content) {
            Ok(r) => r,
            _ => return None,
        };

        let tasks: Value = match serde_json::from_str(&file_content) {
            Ok(tasks) => tasks,
            _ => return None,
        };

        let tasks = match tasks["tasks"].as_array() {
            Some(tasks) => tasks,
            None => return None,
        };

        for task in tasks {
            let task_id = match task.get("id") {
                Some(id) => match id.as_u64() {
                    Some(id) => id,
                    None => return None,
                },
                None => return None,
            };

            // reference the original copy
            if task_id == *id as u64 {
                return match serde_json::from_value::<Task>(task.clone()) {
                    Ok(task) => Some(task),
                    _ => return None,
                };
            }
        }

        None
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
            Err(e) => return Err(e),
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
        reset_file(&mut task_file)?;

        // write back to file
        match task_file.write_all(data.as_bytes()) {
            Ok(_) => Ok(String::from("Task created Successfully")),
            Err(e) => return Err(e),
        }
    }

    // fn delete(&self, task: &mut Task) -> Task {}
}
