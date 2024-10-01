mod err;
use err::{ParseErr, ReadErr};

use json::JsonValue;
pub use json::{parse, stringify};
pub use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    /// Get a json formatted todo list from file through the path
    /// given as parameter.
    /// Return an dynamic error depending on the operation:
    /// reading error (ReadErr) for the file operations
    /// parsing error (ParseErr) for the content operations
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        use std::{fs::File, io::Read};

        // Open the file from the given path.
        // Return a reading error if the file could not be opened.
        let mut file = match File::open(path) {
            Err(e) => {
                return Err(Box::new(ReadErr {
                    child_err: Box::new(e),
                }))
            }
            Ok(f) => f,
        };

        // Read the file and store the content in a new String.
        // Return a reading error if the file could not be read.
        let mut content = String::new();
        if let Err(e) = file.read_to_string(&mut content) {
            return Err(Box::new(ReadErr {
                child_err: Box::new(ParseErr::Malformed(Box::new(e))),
            }));
        };

        // Return a parsing error if the file is empty.
        if content.trim().is_empty() {
            return Err(Box::new(ParseErr::Empty));
        };

        // Parse the file content in json format.
        // Return a parsing error if the content cannot be parsed.
        let parsed = match parse(&content) {
            Err(e) => return Err(Box::new(ParseErr::Malformed(Box::new(e)))),
            Ok(json) => json
        };

        // Extract the title field from the parsed content.
        let title = match &parsed["title"] {
            JsonValue::Short(title) => title.to_string(),
            JsonValue::String(title) => title.clone(),
            _ => unreachable!(),
        };

        // Extract the tasks field from the parsed content
        // one after another taking the id, then the description
        // and finally the level in a temporary collector.
        let tasks = match &parsed["tasks"] {
            JsonValue::Array(tasks) => {
                let mut task_list = Vec::new();

                for task in tasks {
                    let id = match task["id"] {
                        JsonValue::Number(id) => id.as_fixed_point_u64(0).unwrap_or(0) as u32,
                        _ => unreachable!(),
                    };

                    let description = match &task["description"] {
                        JsonValue::Short(desc) => desc.to_string(),
                        JsonValue::String(desc) => desc.clone(),
                        _ => unreachable!(),
                    };

                    let level = match task["level"] {
                        JsonValue::Number(level) => level.as_fixed_point_u64(0).unwrap_or(0) as u32,
                        _ => unreachable!(),
                    };

                    task_list.push(Task { id, description, level });
                }

                task_list
            }
            _ => unreachable!(),
        };

        Ok(TodoList { title, tasks })
    }
}
