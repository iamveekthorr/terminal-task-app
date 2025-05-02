use serde_json::{json, Value};
use std::{
    any::type_name,
    fs::{self, OpenOptions},
    io::{self, Seek, Write},
};

pub fn create_json_data(file_content: &String) -> Result<Value, io::Error> {
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

fn reset_file(task_file: &mut fs::File) -> Result<(), io::Error> {
    task_file
        .seek(io::SeekFrom::Start(0))
        .and_then(|_| task_file.set_len(0))
        .and_then(|_| task_file.flush())
}

pub fn write_json_to_file(
    file: &mut fs::File,
    json_data: &serde_json::Value,
) -> Result<(), io::Error> {
    let data = convert_to_json_string(json_data)?;

    reset_file(file)?;

    file.write_all(data.as_bytes())
}

pub fn convert_to_json_string(json_data: &Value) -> Result<String, io::Error> {
    let data = serde_json::to_string_pretty(json_data).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Error serializing JSON: {}", e),
        )
    })?;
    Ok(data)
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

pub fn setup_test_file(content: &str) -> std::io::Result<()> {
    let mut file = fs::File::create("test_tasks.json")?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn cleanup_test_file() {
    let _ = fs::remove_file("test_tasks.json");
}
