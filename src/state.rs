use std::io::{stdin, stdout, Write};
use std::collections::BTreeMap;
use std::num::IntErrorKind;

use crate::task::Task;

pub struct State {
    pub tasks: BTreeMap<u8, Task>, 
    pub amount: u8,
    pub completed: u8,
}

pub fn add_task(state: &mut State) {
    println!("Add new task.");
    print!("Name: ");

    let mut name = String::new();

    let _ = stdout().flush();
    stdin().read_line(&mut name).expect("Invalid string...");

    if let Some('\n') = name.chars().next_back() {
        name.pop();
    }

    if let Some('\r') = name.chars().next_back() {
        name.pop();
    }

    let task: Task = Task {
        name: name.to_owned(),
        complete: false
    };

    state.amount += 1;
    state.tasks.insert(state.amount, task);
}

pub fn print_tasks(state: &mut State) {
    if state.tasks.len() == 0 {
        println!("No tasks have been added.");
        return;
    }

    println!("Tasks:");
    for (id, task) in &state.tasks {
        println!("{}. Completed: {} | Name: {}", id, task.complete, task.name);
    }
    print_status(state);
}

pub fn remove_task(state: &mut State) {

    let id: u8 = match user_input_task_id(state) {
        Some(id) => id,
        None => {
            println!("Failed to get ID...");
            return;
        }
    };

    let task = match state.tasks.remove(&id) {
        Some(task) => task,
        None => {
            println!("Faild to remove task...");
            return;
        }
    };

    if task.complete {
        state.completed -= 1;
    }

    state.amount -= 1;
    println!("Task removed successfully...");
}

pub fn mark_completed(state: &mut State) {
    let id: u8 = match user_input_task_id(state) {
        Some(id) => id,
        None => {
            println!("Failed to get ID...");
            return;
        }
    };

    let task = match state.tasks.get_mut(&id) {
        Some(task) => task,
        None => {
            println!("Faild to get task...");
            return;
        }
    };

    task.complete = true;
}

pub fn mark_uncompleted(state: &mut State) {
    let id: u8 = match user_input_task_id(state) {
        Some(id) => id,
        None => {
            println!("Failed to get ID...");
            return;
        }
    };

    let task = match state.tasks.get_mut(&id) {
        Some(task) => task,
        None => {
            println!("Faild to get task...");
            return;
        }
    };

    task.complete = false;
}

fn print_status(state: &mut State) {
    println!("Completed: {}/{}", state.completed, state.amount);
}

fn user_input_task_id(state: &mut State) -> Option<u8> {
    print_tasks(state);

    if state.amount == 0 {
        return None;
    }

    print!("Select task (ID): ");
    let _ = stdout().flush();

    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(..) => (),
        Err(..) => println!("Failed to read input...")
    }

    let id: u8 = match input.trim().parse() {
        Ok(id) => {
            if id > state.amount {
                println!("ID is out of range...");
                return None;
            }
            id
        },
        Err(error) => {
            match error.kind() {
                IntErrorKind::InvalidDigit => println!("Input not an integer..."),
                IntErrorKind::PosOverflow => println!("ID to big..."),
                IntErrorKind::Empty => println!("Input is empty..."),
                IntErrorKind::NegOverflow => println!("ID to small..."),
                IntErrorKind::Zero => println!("ID can't be zero..."),
                _ => println!("Unknown error occoured...")
            }
            return None;
        }
    };
    return Some(id);
}
