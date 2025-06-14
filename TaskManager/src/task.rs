#[derive(Debug)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
}

pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: Status,
}

impl Task {
    pub fn new(id: u32, title: String, description: String) -> Self {
        Task {
            id,
            title,
            description,
            status: Status::Pending,
        }
    }

    pub fn update_status(&mut self, new_status: Status) {
        self.status = new_status;
    }

    pub fn mark_completed(&mut self) {
        self.status = Status::Completed;
    }

    pub fn is_completed(&self) -> bool {
        matches!(self.status, Status::Completed)
    }

    pub fn display(&self) -> String {
        format!(
            "Task ID: {}, Title: {}, Description: {}, Status: {:?}",
            self.id, self.title, self.description, self.status
        )
    }
}