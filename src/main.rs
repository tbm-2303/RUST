mod task;
mod storage;
mod commands;

use std::env;
use std::error::Error;
use commands::*;
use storage::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <command> [arguments]");
        eprintln!("Commands: add, list, update, delete, status");
        return Ok(());
    }

    let mut tasks = load_tasks()?; // Now returns Result
    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: cargo run add \"task description\"");
            } else {
                let description = args[2].clone();
                let id = add_task(&mut tasks, description);
                save_tasks(&tasks)?;
                println!("Task {id} added.");
            }
        }

        "list" => {
            list_tasks(&tasks);
        }

        "update" => {
            if args.len() < 4 {
                eprintln!("Usage: cargo run update <id> \"new description\"");
            } else {
                let id: u32 = args[2].parse().unwrap_or_else(|_| {
                    eprintln!("invalid task ID. Must be a number.");
                    std::process::exit(1);
                });
                let new_desc = args[3].clone();
                match update_task(&mut tasks, id, new_desc) {
                    Ok(_) => {
                        save_tasks(&tasks)?;
                        println!("Task {id} updated.");
                    }
                    Err(e) => eprintln!("{e}"),
                }
            }
        }

        "delete" => {
            if args.len() < 3 {
                eprintln!("Usage: cargo run delete <id>");
            } else {
                let id: u32 = args[2].parse().unwrap_or_else(|_| {
                    eprintln!("Invalid task ID. Must be a number.");
                    std::process::exit(1);
                });
                match delete_task(&mut tasks, id) {
                    Ok(_) => {
                        save_tasks(&tasks)?;
                        println!("Task {id} deleted.");
                    }
                    Err(e) => eprintln!("{e}"),
                }
            }
        }

        "status" => {
            if args.len() < 4 {
                eprintln!("Usage: cargo run status <id> <Todo|InProgress|Done>");
            } else {
                let id: u32 = args[2].parse().unwrap_or_else(|_| {
                    eprintln!("Invalid task ID. Must be a number.");
                    std::process::exit(1);
                });
                let new_status = &args[3];
                match update_status(&mut tasks, id, new_status) {
                    Ok(_) => {
                        save_tasks(&tasks)?;
                        println!("Task {id} status updated to {new_status}.");
                    }
                    Err(e) => eprintln!("{e}"),
                }
            }
        }

        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Commands: add, list, update, delete, status");
        }
    }

    Ok(())
}
