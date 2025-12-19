use std::io::stdin;

use crate::{
    actions::create_todo,
    models::{clap::State, todo::Todo, todo_status::TodoStatus},
};

pub fn create_state() {
    clearscreen::clear().expect("failed to clear screen");

    let mut todo = Todo::new();

    println!("Please enter the name of the task to create:");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read line");
    State::new(&input);

    todo.name = input.trim().to_string();

    clearscreen::clear().expect("failed to clear screen");
    println!("Please enter the state (Pending,Doing,Done):");

    input = String::new();
    stdin().read_line(&mut input).expect("failed to read line");

    todo.status = TodoStatus::new(&input.trim());

    create_todo(&todo.name, todo.status);
}
