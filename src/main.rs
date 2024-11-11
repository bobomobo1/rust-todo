use serde::{Deserialize, Serialize};
use std::{fs, fs::OpenOptions, io::Write, path::Path};
use clap::{Parser, Subcommand};

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    title: String,
    completed: bool,
}

impl TodoItem {
    fn new(title: String) -> Self {
        TodoItem {
            title,
            completed: false,
        }
    }
}

#[derive(Parser)]
#[command(name = "Todo CLI")]
#[command(about = "A simple command-line to-do list", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { title: String },
    #[command(about = "List all tasks")]
    List,
    Complete { index: usize },
    Remove { index: usize },
}

const FILE_PATH: &str = "todos.json";

fn load_todos() -> Vec<TodoItem> {
    if Path::new(FILE_PATH).exists() {
        let data = fs::read_to_string(FILE_PATH).expect("Unable to read file");
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_todos(todos: &[TodoItem]) {
    let data = serde_json::to_string_pretty(todos).expect("Unable to serialize data");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)
        .expect("Unable to open file");
    file.write_all(data.as_bytes()).expect("Unable to write data");
}

fn main() {
    let cli = Cli::parse();
    let mut todos = load_todos();

    match &cli.command {
        Commands::Add { title } => {
            todos.push(TodoItem::new(title.clone()));
            save_todos(&todos);
            println!("Added task: {}", title);
        }
        Commands::List => {
            if todos.is_empty() {
                println!("No tasks found!");
            } else {
                for (index, todo) in todos.iter().enumerate() {
                    let status = if todo.completed { "✓" } else { " " };
                    println!("{} - [{}] {}", index + 1, status, todo.title);
                }
            }
        }
        Commands::Complete { index } => {
            if let Some(todo) = todos.get_mut(*index - 1) {
                todo.completed = true;
                save_todos(&todos);
                println!("Marked task {} as completed", index);
            } else {
                println!("Task {} not found", index);
            }
        }
        Commands::Remove { index } => {
            if *index > 0 && *index <= todos.len() {
                todos.remove(*index - 1);
                save_todos(&todos);
                println!("Removed task {}", index);
            } else {
                println!("Task {} not found", index);
            }
        }
    }
}
