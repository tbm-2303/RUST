use std::fs;
use std::path::Path;
use std::error::Error;
use crate::task::Task;

pub fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    if Path::new("tasks.json").exists() {
        let data = fs::read_to_string("tasks.json")?;
        let tasks: Vec<Task> = serde_json::from_str(&data)?;
        Ok(tasks)
    } else {
        Ok(Vec::new())
    }
}

pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write("tasks.json", data)?;
    Ok(())
}
