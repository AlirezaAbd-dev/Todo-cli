use std::io::stdin;

use crate::{
    cli::{create_state, delete_todo_state, get_todo_state, get_todos_state, update_todo_state},
    models::clap::State,
};

mod actions;
mod cli;
mod models;

fn main() {
    loop {
        clearscreen::clear().expect("failed to clear screen");

        println!("Welcome to Todo-cli Application");
        println!("Choose one of the options bellow:");
        println!("1. Create a new todo");
        println!("2. Update an existing todo");
        println!("3. Delete a todo");
        println!("4. Get all todos");
        println!("5. Get a single todo");
        println!("6. Exit");

        let mut input = String::new();

        stdin().read_line(&mut input).expect("Failed to read line");

        match State::new(&input) {
            State::Create => {
                create_state();
            }
            State::Update => {
                update_todo_state();
            }
            State::Delete => delete_todo_state(),
            State::GetAll => {
                get_todos_state();
            }
            State::GetSingle => get_todo_state(),
            State::Exit => {
                clearscreen::clear().expect("failed to clear screen");
                break;
            }
        }
    }
}
