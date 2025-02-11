mod db;

use db::Database;
use std::io::{self, Write};
use std::process;

#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn new(id: i32, title: String, done: bool) -> Task {
        Task { id, title, done }
    }

    fn display(&self) -> String {
        let status = if self.done { "[✓]" } else { "[ ]" };
        format!("{} {} - {}", status, self.id, self.title)
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn get_task_title() -> String {
    println!("Enter a new task:");
    read_input()
}

fn get_task_id() -> String {
    println!("Enter task ID:");
    read_input()
}

fn main() {
    let db = match Database::new("tasks.db") {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            process::exit(1);
        }
    };

    println!("\nTODO Application");
    println!("---------------");
    println!("1. Add a task");
    println!("2. List tasks");
    println!("3. Mark task as done");
    println!("4. Delete a task");
    println!("5. Exit");

    loop {
        print!("\nEnter your choice (1-5): ");
        io::stdout().flush().unwrap();

        let choice = read_input();

        match choice.as_str() {
            "1" => {
                let task_title = get_task_title();
                if !task_title.is_empty() {
                    db.add_task(&task_title).expect("Failed to add task");
                } else {
                    println!("Task cannot be empty!");
                }
            }
            "2" => match db.list_tasks() {
                Ok(tasks) => {
                    if tasks.is_empty() {
                        println!("No tasks found.");
                    } else {
                        println!("Your tasks:");
                        for task in tasks {
                            println!("{}", task.display());
                        }
                    }
                }
                Err(e) => eprintln!("Error listing tasks: {}", e),
            },
            "3" => {
                let task_id = get_task_id();
                if let Ok(id) = task_id.parse::<i32>() {
                    db.mark_task_done(id).expect("Failed to mark task as done");
                } else {
                    println!("Invalid task ID.");
                }
            }
            "4" => {
                let task_id = get_task_id();
                if let Ok(id) = task_id.parse::<i32>() {
                    db.delete_task(id).expect("Failed to delete task");
                } else {
                    println!("Invalid task ID.");
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice."),
        }
    }
}
