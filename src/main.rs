mod task;

use chrono::{DateTime};
use chrono::offset::Local;

use serde_json::json;

use std::fmt::Display;
use std::fs;
use std::io::{self, Write};


use task::{Task, TaskList};

struct Config {
}
impl Config {
    const TASK_FILE: &'static str = "./tasks.json";
}
fn main() {
    let mut task_list = load_task_list();
    let mut buffer = String::new();
    loop {
        match prompt_user().as_str() {
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
        Err (_) => return String::new(),
    };
}

fn prompt_user_to_add_task(task_list: &mut TaskList<String>) {
    loop {
        print!("Enter task name: ");
        io::stdout().flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok (_) => {
                task_list.add_task(String::from(input));
                break;
            }
            Err (_) => continue,
        }
    }
}

fn prompt_user_to_delete_task(task_list: &mut TaskList<String>) {
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

fn print_user_task_list(task_list: &TaskList<String>) {
    task_list.print_list();
}

fn load_task_list() -> TaskList<String> {
    let contents: String;
    match fs::read_to_string(Config::TASK_FILE) {
        Ok(s) => contents = s,
        Err(err) => contents = String::new(),
    }
    match serde_json::from_str(contents.as_str()) {
        Ok(t) => t,
        Err(err) => TaskList::<String>::load_task_list(),
    }
}

fn write_task_list (task_list: &TaskList<String>) {
    match serde_json::to_string(&task_list) {
        Ok(s) => {
            match fs::write(Config::TASK_FILE, s) {
                Ok(_) => {},
                Err(_) => {},
            }
        },
        Err(_) => {},
    }
}