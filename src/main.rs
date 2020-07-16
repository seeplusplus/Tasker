mod task;

use chrono::{DateTime};
use chrono::offset::Local;
use std::io::{self, Write};
use task::{Task, TaskList};


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
