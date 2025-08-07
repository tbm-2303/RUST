use crate::task::{Task, TaskStatus};

pub fn add_task(tasks: &mut Vec<Task>, description: String) -> u32 {
    let id = next_id(tasks);
    tasks.push(Task::new(id, description));
    id
}

pub fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks.");
        return;
    }

    for t in tasks {
        println!("{}: [{}] {}", t.id, status_to_str(&t.status), t.description);
    }
}

pub fn update_task(tasks: &mut Vec<Task>, id: u32, new_description: String) -> Result<(), String> {
    if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
        t.description = new_description;
        Ok(())
    } else {
        Err(format!("Task {id} not found"))
    }
}

pub fn delete_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), String> {
    let before = tasks.len();
    tasks.retain(|t| t.id != id);
    if tasks.len() == before {
        Err(format!("Task {id} not found"))
    } else {
        Ok(())
    }
}

pub fn update_status(tasks: &mut Vec<Task>, id: u32, new_status: &str) -> Result<(), String> {
    if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
        t.status = match new_status {
            "Todo" => TaskStatus::Todo,
            "InProgress" => TaskStatus::InProgress,
            "Done" => TaskStatus::Done,
            _ => return Err(format!("Invalid status: {}. Use Todo, InProgress, or DOne.", new_status)),
        };
        Ok(())
    } else {
        Err(format!("Task {id} not found"))
    }
}

fn next_id(tasks: &Vec<Task>) -> u32 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

fn status_to_str(s: &TaskStatus) -> &'static str {
    match s {
        TaskStatus::Todo => "Todo",
        TaskStatus::InProgress => "InProgress",
        TaskStatus::Done => "Done",
    }
}
