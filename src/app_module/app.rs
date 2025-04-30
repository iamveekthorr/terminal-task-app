pub use crate::tasks_module::tasks;
use crate::tasks_module::tasks_definitions::{Task, TaskTrait};

pub struct App {
    pub command: String,
}

#[derive(Debug)]
enum Command {
    CREATE,
    UPDATE,
    DELETE,
    LIST,
}

impl Command {
    fn from(command: &str) -> Result<Self, &'static str> {
        match command.to_lowercase().as_str() {
            "add" | "create" => Ok(Command::CREATE),
            "update" => Ok(Command::UPDATE),
            "delete" => Ok(Command::DELETE),
            "list" => Ok(Command::LIST),
            _ => Err("Invalid command"),
        }
    }
}

enum CommandRequest {
    Create { description: String },
    Update { id: u32, description: String },
    Delete { id: u32 },
    List,
}

fn parse_command(mut args: impl Iterator<Item = String>) -> Result<CommandRequest, &'static str> {
    args.next(); // skip binary name

    let command = match args.next() {
        Some(cmd) => cmd,
        None => return Err("No command provided"),
    };

    match Command::from(&command) {
        Ok(Command::CREATE) => {
            let description = args.next().ok_or("Missing task description")?;
            Ok(CommandRequest::Create { description })
        }
        Ok(Command::UPDATE) => {
            let id_str = args.next().ok_or("Missing task ID")?;
            let id = id_str.parse().map_err(|_| "Invalid task ID")?;
            let description = args.next().ok_or("Missing task description")?;
            Ok(CommandRequest::Update { id, description })
        }
        Ok(Command::DELETE) => {
            let id_str = args.next().ok_or("Missing task ID")?;
            let id = id_str.parse().map_err(|_| "Invalid task ID")?;
            Ok(CommandRequest::Delete { id })
        }
        Ok(Command::LIST) => Ok(CommandRequest::List),
        _ => Err("Unknown or missing command"),
    }
}

impl App {
    pub fn run(args: impl Iterator<Item = String>) -> Result<(), &'static str> {
        let request = parse_command(args)?;
        let mut task = Task::new(); // replace 1 with dynamic id later

        match request {
            CommandRequest::Create { description } => {
                task.description = description;
                task.create().map_err(|_| "Error creating task")?;
                println!("Task created");
            }
            CommandRequest::Update { id, description } => {
                let updated = task
                    .update(&id, &Some(description))
                    .map_err(|_| "Error updating task")?;

                let updated =
                    serde_json::to_string_pretty(&updated).map_err(|_| "Error serializing JSON")?;

                println!("Task updated: {}", updated);
            }
            CommandRequest::Delete { id } => {
                // TODO: implement Task::delete
                println!("(Simulating) Task {} deleted", id);
            }
            CommandRequest::List => {
                let tasks = task.list().map_err(|_| "Something happened")?;

                // convert to json string for easy printing
                let tasks =
                    serde_json::to_string_pretty(&tasks).map_err(|_| "Error serializing JSON")?;

                println!("{}", tasks);
            }
        }

        Ok(())
    }
}
