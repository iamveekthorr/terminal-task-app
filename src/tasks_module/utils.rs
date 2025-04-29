use serde_json::{json, Value};
use std::{
    any::type_name,
    fs::{self, OpenOptions},
    io::{self, Seek, Write},
};

pub fn create_json_data(file_content: String) -> Result<Value, io::Error> {
    let json_data: Value = if file_content.trim().is_empty() {
        let json_string = json!({
            "tasks": []
        });

        match serde_json::from_value(json_string) {
            Ok(data) => data,
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Error parsing JSON: {}", e),
                ));
            }
        }
    } else {
        match serde_json::from_str(&file_content) {
            Ok(data) => data,
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Error parsing JSON: {}", e),
                ));
            }
        }
    };
    Ok(json_data)
}

pub fn reset_file(task_file: &mut fs::File) -> Result<(), io::Error> {
    task_file.seek(io::SeekFrom::Start(0))?;
    task_file.set_len(0)?;
    task_file.flush()?;

    //  return nothing
    Ok(())
}

pub fn _print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

pub fn open_file() -> Result<fs::File, io::Error> {
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
