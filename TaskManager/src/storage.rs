use std::collections::HashMap;
use crate::task::Task;
use crate::task::Status;


pub struct Storage {
    pub tasks: HashMap<u32,Task>,
}

impl Storage {
    pub fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    pub fn view_all_tasks(&self)  {
        for task in self.tasks.values() {
            println!("{}", task.display());
        }
    }

    pub fn mark_completed(&mut self, task_id: u32) -> Result<(), String> {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.mark_completed();
            Ok(())
        } else {
            Err(format!("Task with ID {} not found.", task_id))
        }
    }

    pub fn delete_task(&mut self, task_id: u32) -> Result<(), String> {
        match self.tasks.remove(&task_id) {
            Some(_) => Ok(()),
            None => Err(format!("Task with ID {} not found.", task_id)),
        }
    }


    pub fn new() -> Self {
        Storage {
            tasks: HashMap::new(),
        }
    }


}


