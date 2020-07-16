use chrono::{DateTime, Local};

enum TaskStatus {
    Pending,
    Completed
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::Completed => write!(f, "Completed"),
        }
    }
}   

pub struct Task {
    status: TaskStatus,
    date_time_created: DateTime<Local>,
    task_description: String
}

impl Task {
    fn create_task (from_task_description: &str) -> Task {
        Task { 
            status: TaskStatus::Pending, 
            date_time_created: Local::now(), 
            task_description: String::from(from_task_description)
        }
    }
}
pub struct TaskList {
    task_vec: Vec<Task>
}

impl TaskList {
    pub fn load_task_list() -> TaskList {
        TaskList { task_vec: Vec::new() }
    }
    pub fn print_list(&self) {
        for (idx, task) in self.task_vec.iter().enumerate() {
            println!("[{}] Description: {}, Status: {}, Added: {}", idx, task.task_description, task.status, task.date_time_created);
        }
    }
    pub fn add_task(&mut self, task_description: &str) {
        self.task_vec.push(Task::create_task(task_description));
    }
    pub fn delete_task(&mut self, idx: usize) -> bool {
        println!("{}", idx);
        println!("{}", idx >= (self.task_vec.len() - 1));
        if idx >= (self.task_vec.len() - 1) { 
            println!("{}", self.task_vec.len());
            return false;
        }
        self.task_vec.remove(idx);
        true
    }
}