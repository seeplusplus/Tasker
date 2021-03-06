use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
enum TaskStatus {
    Pending,
    Completed,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::Completed => write!(f, "Completed"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Task<T: Display> {
    status: TaskStatus,
    date_time_created: DateTime<Local>,
    task_description: T,
}

impl<T: Display> Task<T> {
    fn from(from_task_description: T) -> Task<T> {
        Task {
            status: TaskStatus::Pending,
            date_time_created: Local::now(),
            task_description: from_task_description,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaskList<T: Display> {
    task_vec: Vec<Task<T>>,
}

impl<T: Display> TaskList<T> {
    pub fn new() -> TaskList<T> {
        TaskList {
            task_vec: Vec::new(),
        }
    }
    pub fn print(&self) {
        for (idx, task) in self.task_vec.iter().enumerate() {
            println!(
                "[{}] Description: {}, Status: {}, Added: {}",
                idx, task.task_description, task.status, task.date_time_created
            );
        }
    }
    pub fn add(&mut self, task_description: T) {
        self.task_vec.push(Task::from(task_description));
    }
    pub fn remove(&mut self, idx: usize) -> bool {
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
