mod persistence;
mod queue;
mod todo;

use clap::{Parser, Subcommand};
use todo::Todo;

use crate::persistence::{load_from_file, save_to_file};
use crate::queue::Queue;

const DATA_FILE: &str = "todos.bin";

#[derive(Parser)]
#[command(name = "todo", about = "A simple persistent todo queue", version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
    Done,
}

fn main() {
    let cli = Cli::parse();

    let mut queue: Queue<Todo> = load_from_file(DATA_FILE).unwrap_or_else(|e| {
        eprintln!("Error loading todos: {}", e);
        Queue::new()
    });

    match &cli.command.unwrap_or(Commands::List) {
        Commands::Add { description } => {
            let next_id = queue.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            let todo = Todo::new(next_id, description.clone());
            queue.enqueue(todo);
            save_to_file(&queue, DATA_FILE).unwrap_or_else(|e| {
                eprintln!("Error saving todos: {}", e);
            });
            println!("Added task: \"{}\"", description);
        }
        Commands::List => {
            if queue.is_empty() {
                println!("No pending tasks.");
            } else {
                println!("Pending tasks ({}):", queue.len());
                for (i, todo) in queue.iter().enumerate() {
                    println!("{}. [{}] {}", i + 1, todo.id, todo.description);
                }
            }
        }
        Commands::Done => {
            match queue.peek() {
                Some(_) => {
                    let todo = queue.dequeue().unwrap();
                    save_to_file(&queue, DATA_FILE).unwrap_or_else(|e| {
                        eprintln!("Error saving todos: {}", e);
                    });
                    println!("Completed task: \"{}\"", todo.description);
                }
                None => {
                    println!("No pending tasks to complete.");
                }
            }
        }
    }
}
