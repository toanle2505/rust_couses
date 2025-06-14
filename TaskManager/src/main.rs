mod task;
mod storage;

use crate::task::{Task, Status};
use crate::storage::Storage;
use std::io::{self, Write};
fn main() {

    let mut storage = Storage::new();
    let mut input = String::new();

    loop {
        println!("Task Manager");
        println!("1. Add Task");
        println!("2. View All Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Delete Task");
        println!("5. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let choice: u32 = input.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                let mut title = String::new();
                let mut description = String::new();

                print!("Enter task title: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut title).unwrap();

                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut description).unwrap();

                let task_id = storage.tasks.len() as u32 + 1;
                let task = Task::new(task_id, title.trim().to_string(), description.trim().to_string());
                storage.add_task(task);
            }
            2 => {
                storage.view_all_tasks();
            }
            3 => {
                print!("Enter task ID to mark as completed: ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let task_id: u32 = input.trim().parse().unwrap_or(0);
                
                match storage.mark_completed(task_id) {
                    Ok(_) => println!("Task marked as completed."),
                    Err(e) => println!("{}", e),
                }
            }
            4 => {
                print!("Enter task ID to delete: ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let task_id: u32 = input.trim().parse().unwrap_or(0);
                
                match storage.delete_task(task_id) {
                    Ok(_) => println!("Task deleted."),
                    Err(e) => println!("{}", e),
                }
            }
            5 => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}
