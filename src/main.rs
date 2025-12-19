use std::io::stdin;

use crate::{actions::create_todo, cli::create_state, models::clap::State};

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
                println!("Update")
            }
            State::Delete => {
                println!("Delete")
            }
            State::GetAll => {
                println!("Get All")
            }
            State::GetSingle => {
                println!("Get Single")
            }
            State::Exit => {
                clearscreen::clear().expect("failed to clear screen");
                break;
            }
        }
    }
}
