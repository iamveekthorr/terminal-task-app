use std::env;

use tasks_module::tasks_definitions::{Task, TaskTrait};

mod tasks_module;

#[derive(Debug)]
enum Command {
    CREATE,
    UPDATE,
    DELETE,
    LIST,
}

impl Command {
    fn from(command: &str) -> Self {
        match command.to_lowercase().as_str() {
            "add" => Command::CREATE,
            "update" => Command::UPDATE,
            "delete" => Command::DELETE,
            "list" => Command::LIST,
            _ => panic!("Invalid command"),
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        panic!("Please provide a task id");
    }

    println!("{:?}", args);

    let command = args[1].as_str();

    // shadow the older variables
    let command = Command::from(command);

    match command {
        Command::CREATE => {
            let task_description = args[2].as_str();
            let task_description = String::from(task_description);
            let task = Task::new(1, task_description);

            match task.create() {
                Ok(res) => println!("{}", res),
                Err(e) => {
                    panic!("Error creating task: {}", e);
                }
            };
        }
        Command::UPDATE => {
            let task_description = args[3].as_str();
            let task_description = String::from(task_description);
            let task_id = 1;

            match Task::update(&task_id, Some(task_description)) {
                Ok(res) => {
                    println!("Task has been updated successfully: {:?}", res)
                }
                Err(e) => {
                    panic!("Error updating task: {}", e);
                }
            };
        }
        Command::DELETE => {
            // let task = task.delete(&mut task);
            // println!("Task deleted: {:?}", task);
        }
        Command::LIST => {
            // let task = task.get();
            // match task {
            //     Some(task) => println!("Task: {:?}", task),
            //     None => println!("No tasks found"),
            // }
        }
    }
}
