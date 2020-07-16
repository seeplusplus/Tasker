// program runs and shows a list of tasks
// DONE:
//  + add a task
//  + delete a task
// TODO:
//  + mark a task as complete
//  + view tasks
//     + pending or completed
//  + save on quit
//  + load on run
//  + panic handling on delete
use chrono::{DateTime};
use chrono::offset::Local;
use std::io::{self, Write};

fn main() {
    let mut task_list = TaskList::load_task_list();
    let mut buffer = String::new();
    loop {
        match prompt_user().as_str() {
            "a" => prompt_user_to_add_task(&mut task_list),
            "p" => task_list.print_list(),
            "d" => prompt_user_to_delete_task(&mut task_list),
            "q" => break,
            _ => continue,
        }
        buffer.clear();
    }
}
fn prompt_user() -> String {
    println!("Menu:\n(a)dd a task\n(p)rint the list of tasks\n(q)uit");
    let mut input = String::new(); 
    match io::stdin().read_line(&mut input) {
        Ok(_) => return String::from(input.trim()),
        Err (_) => return String::new(),
    };
}

fn prompt_user_to_add_task(task_list: &mut TaskList) {
    loop {
        print!("Enter task name: ");
        io::stdout().flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok (_) => {
                task_list.add_task(input.trim());
                break;
            }
            Err (_) => continue,
        }
    }
}

fn prompt_user_to_delete_task(task_list: &mut TaskList) {
    loop {
        task_list.print_list();
        print!("Task index to delete: ");
        io::stdout().flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok (_) => {
                let mut task_deleted = false;
                match input.trim().parse::<usize>() {
                    Ok (n) => 
                    {
                        task_list.delete_task(n);
                        task_deleted = true;
                    }
                    Err (err) => {
                        println!("{}", err);
                        continue
                    }
                }
                if task_deleted {
                    break;
                }
            }
            Err (_) => continue,
        }
    }
}

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

struct Task {
    status: TaskStatus,
    date_time_created: DateTime<Local>,
    task_description: String
}

struct TaskList {
    task_vec: Vec<Task>
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

impl TaskList {
    fn load_task_list() -> TaskList {
        TaskList { task_vec: Vec::new() }
    }
    fn print_list(&self) {
        for (idx, task) in self.task_vec.iter().enumerate() {
            println!("[{}] Description: {}, Status: {}, Added: {}", idx, task.task_description, task.status, task.date_time_created);
        }
    }
    fn add_task(&mut self, task_description: &str) {
        self.task_vec.push(Task::create_task(task_description));
    }
    fn delete_task(&mut self, idx: usize) {
        self.task_vec.remove(idx);
    }
}