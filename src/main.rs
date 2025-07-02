mod db;

use db::Database;
use std::io::{self, Write};
use std::process;
use chrono::{Local, Datelike};

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

    fn display(&self, serial_number: usize) -> String {
        let status = if self.done { "✅" } else { "⭕" };
        let now = Local::now();
        let day = match now.day() {
            1 | 21 | 31 => format!("{}st", now.day()),
            2 | 22 => format!("{}nd", now.day()),
            3 | 23 => format!("{}rd", now.day()),
            _ => format!("{}th", now.day()),
        };
        let month = now.format("%b").to_string();
        let time = now.format("(%I:%M%p)").to_string().to_lowercase();
        
        format!("{}  {}. {} - {} {} {}", status, serial_number, self.title, day, month, time)
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
    println!("\n📝 Enter a new task:");
    print!("➤ ");
    io::stdout().flush().unwrap();
    read_input()
}

fn get_task_id() -> String {
    println!("\n🔢 Enter task serial number:");
    print!("➤ ");
    io::stdout().flush().unwrap();
    read_input()
}

fn print_banner() {
    println!("___________           .___        .____    .__          __   ");
    println!("\\__    ___/___     __| _/____     |    |   |__| _______/  |_ ");
    println!("  |    | /  _ \\   / __ |/  _ \\    |    |   |  |/  ___/\\   __\\");
    println!("  |    |(  <_> ) / /_/ (  <_> )   |    |___|  |\\___ \\  |  |  ");
    println!("  |____| \\____/  \\____ |\\____/    |_______ \\__|/____  > |__|  ");
    println!("                      \\/                  \\/        \\/       ");
    println!();
}

fn display_tasks(tasks: &[Task]) {
    let width = 62; // Increased width to extend more to the right
    
    println!();
    println!("╔{}╗", "═".repeat(width - 2));
    print_centered_with_borders("📝 TODO LIST", width);
    println!("╠{}╣", "═".repeat(width - 2));
    
    if tasks.is_empty() {
        print_left_aligned_with_borders("", width);
        print_left_aligned_with_borders("🌟 No tasks yet! Add your first task to get started", width);
        print_left_aligned_with_borders("", width);
    } else {
        print_left_aligned_with_borders("", width);
        for (index, task) in tasks.iter().enumerate() {
            let task_display = task.display(index + 1);
            print_left_aligned_with_borders(&task_display, width);
        }
        print_left_aligned_with_borders("", width);
    }
    
    println!("╚{}╝", "═".repeat(width - 2));
    println!();
}

fn print_centered(text: &str, width: usize) {
    let text_len = text.chars().count(); // Better unicode handling
    if text_len >= width {
        println!("{}", text);
    } else {
        let padding = (width - text_len) / 2;
        println!("{:padding$}{}", "", text, padding = padding);
    }
}

fn print_centered_with_borders(text: &str, width: usize) {
    let text_len = text.chars().count();
    let inner_width = width - 4; // Account for "║  " and "  ║"
    
    if text_len >= inner_width {
        println!("║  {:<width$}  ║", text, width = inner_width);
    } else {
        let padding = (inner_width - text_len) / 2;
        let remaining = inner_width - text_len - padding;
        println!("║  {:padding$}{}{:remaining$}  ║", "", text, "", 
                padding = padding, remaining = remaining);
    }
}

fn print_left_aligned_with_borders(text: &str, width: usize) {
    let inner_width = width - 4; // Account for "║  " and "  ║"
    if text.is_empty() {
        println!("║{:width$}║", "", width = width - 2);
    } else {
        println!("║  {:<width$}  ║", text, width = inner_width);
    }
}

fn print_separator(width: usize) {
    println!("{}", "=".repeat(width));
}

fn main() {
    let db = match Database::new("tasks.db") {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            process::exit(1);
        }
    };

    // Show the banner once at startup
    print_banner();

    loop {
        // Show current tasks
        match db.list_tasks() {
            Ok(tasks) => display_tasks(&tasks),
            Err(e) => eprintln!("Error displaying tasks: {}", e),
        }
        
        // Menu options with better formatting
        println!("╔══════════════════════════════════════════════════════════╗");
        println!("║                      CHOOSE AN OPTION                   ║");
        println!("╠══════════════════════════════════════════════════════════╣");
        println!("║  1️⃣  Add a new task                                      ║");
        println!("║  2️⃣  Mark task as done                                   ║");
        println!("║  3️⃣  Delete a task                                       ║");
        println!("║  4️⃣  Exit                                                ║");
        println!("╚══════════════════════════════════════════════════════════╝");

        print!("\n🎯 Enter your choice (1-4): ");
        io::stdout().flush().unwrap();

        let choice = read_input();

        match choice.as_str() {
            "1" => {
                let task_title = get_task_title();
                if !task_title.is_empty() {
                    db.add_task(&task_title).expect("Failed to add task");
                } else {
                    println!("❌ Task cannot be empty!");
                }
            }
            "2" => {
                match db.list_tasks() {
                    Ok(tasks) => {
                        if !tasks.is_empty() {
                            let task_id = get_task_id();
                            if let Ok(serial_num) = task_id.parse::<usize>() {
                                if serial_num > 0 && serial_num <= tasks.len() {
                                    let actual_id = tasks[serial_num - 1].id;
                                    db.mark_task_done(actual_id).expect("Failed to mark task as done");
                                } else {
                                    println!("❌ Invalid task number. Please choose between 1 and {}.", tasks.len());
                                }
                            } else {
                                println!("❌ Invalid input. Please enter a number.");
                            }
                        } else {
                            println!("❌ No tasks available to mark as done.");
                        }
                    }
                    Err(e) => eprintln!("Error loading tasks: {}", e),
                }
            }
            "3" => {
                match db.list_tasks() {
                    Ok(tasks) => {
                        if !tasks.is_empty() {
                            let task_id = get_task_id();
                            if let Ok(serial_num) = task_id.parse::<usize>() {
                                if serial_num > 0 && serial_num <= tasks.len() {
                                    let actual_id = tasks[serial_num - 1].id;
                                    db.delete_task(actual_id).expect("Failed to delete task");
                                } else {
                                    println!("❌ Invalid task number. Please choose between 1 and {}.", tasks.len());
                                }
                            } else {
                                println!("❌ Invalid input. Please enter a number.");
                            }
                        } else {
                            println!("❌ No tasks available to delete.");
                        }
                    }
                    Err(e) => eprintln!("Error loading tasks: {}", e),
                }
            }
            "4" => {
                println!("\n🎉 Thanks for using Todo List! 🎉");
                println!("╔══════════════════════════════════════════════════════════╗");
                println!("║                         GOODBYE!                        ║");
                println!("║                    Keep being productive! 🚀             ║");
                println!("╚══════════════════════════════════════════════════════════╝");
                break;
            }
            _ => println!("❌ Invalid choice. Please choose 1-4."),
        }
    }
}
