mod task;

use chrono::offset::Local;
use chrono::DateTime;

use serde_json::json;

use std::env;
use std::fmt::Display;
use std::fs;
use std::io::{self, Write};
use std::path;

use task::{Task, TaskList};

struct Config {}
impl Config {
    fn get_task_file() -> io::Result<path::PathBuf> {
        let mut json_path = env::current_exe()?;
        json_path.pop();
        json_path.push("tasks.json");
        Ok(json_path)
    }
}

fn main() {
    let mut task_list = load_task_list().unwrap_or(TaskList::<String>::new());
    let mut buffer = String::new();
    loop {
        match prompt_user().to_ascii_lowercase().as_str() {
            "a" => prompt_user_to_add_task(&mut task_list),
            "p" => print_user_task_list(&task_list),
            "d" => prompt_user_to_delete_task(&mut task_list),
            "q" => break,
            _ => continue,
        }
        buffer.clear();
    }
    write_task_list(&task_list);
}
fn prompt_user() -> String {
    println!("Menu:\n(a)dd a task\n(p)rint the list of tasks\n(q)uit");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => return String::from(input.trim()),
        Err(_) => return String::new(),
    };
}

fn prompt_user_to_add_task(task_list: &mut TaskList<String>) {
    loop {
        print!("Enter task name: ");
        io::stdout().flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                task_list.add(String::from(input.trim()));
                break;
            }
            Err(_) => continue,
        }
    }
}

fn prompt_user_to_delete_task(task_list: &mut TaskList<String>) {
    loop {
        task_list.print();
        print!("Task index to delete: ");
        io::stdout().flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let mut task_deleted = false;
                match input.trim().parse::<usize>() {
                    Ok(n) => {
                        task_list.remove(n);
                        task_deleted = true;
                    }
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                }
                if task_deleted {
                    break;
                }
            }
            Err(_) => continue,
        }
    }
}

fn print_user_task_list(task_list: &TaskList<String>) {
    task_list.print();
}

fn load_task_list() -> io::Result<TaskList<String>> {
    let contents: String;
    match fs::read_to_string(Config::get_task_file()?) {
        Ok(s) => contents = s,
        Err(err) => contents = String::new(),
    }
    match serde_json::from_str(contents.as_str()) {
        Ok(t) => Ok(t),
        Err(err) => Ok(TaskList::<String>::new()),
    }
}

fn write_task_list(task_list: &TaskList<String>) -> io::Result<()> {
    match serde_json::to_string(&task_list) {
        Ok(s) => match fs::write(Config::get_task_file()?, s) {
            Ok(_) => {}
            Err(_) => {}
        },
        Err(_) => {}
    }
    Ok(())
}
