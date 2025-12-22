use std::io::stdin;

use crate::{
    actions::{create_todo, delete_todo, get_single_todo, get_todos, update_todo},
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

pub fn get_todos_state() {
    clearscreen::clear().expect("failed to clear screen");
    println!("Here is the list of todos:");

    let todos = get_todos();

    match todos {
        Ok(todos) => {
            for todo in todos {
                println!("{}", todo);
            }
        }
        Err(_) => {
            println!("Failed to get todos");
        }
    }

    println!("");
    println!("Please hit the enter to exit to home!");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read line");
}

pub fn get_todo_state() {
    loop {
        clearscreen::clear().expect("failed to clear screen");

        println!("Please enter the id of todo:(for example (1))");

        let mut input = String::new();

        stdin().read_line(&mut input).expect("failed to read line");

        let id = input.trim().parse();

        match id {
            Ok(id) => {
                let todo = get_single_todo(id);

                match todo {
                    Some(todo) => {
                        clearscreen::clear().expect("failed to clear screen");

                        println!("Here is the todo:");
                        println!("");
                        println!("{}", todo);
                        println!("");
                        println!("Please hit the enter to exit to home!");
                        stdin().read_line(&mut input).expect("failed to read line");
                        break;
                    }
                    None => {
                        clearscreen::clear().expect("failed to clear screen");

                        println!("Todo not found");

                        println!("");
                        println!("Please hit the enter to exit to home!");
                        stdin().read_line(&mut input).expect("failed to read line");
                        break;
                    }
                }
            }
            Err(_) => {
                println!("Invalid id");
                println!("Please try again:");
                stdin().read_line(&mut input).expect("failed to read line");
                continue;
            }
        }
    }
}

pub fn update_todo_state() {
    loop {
        clearscreen::clear().expect("failed to clear screen");

        println!("Please enter the id of todo:");

        let mut input = String::new();

        stdin().read_line(&mut input).expect("failed to read line");

        let id: Result<u64, _> = input.trim().parse();

        match id {
            Ok(id) => {
                clearscreen::clear().expect("failed to clear screen");

                let find_todo = get_single_todo(id);

                match find_todo {
                    Some(todo) => {
                        let mut new_todo = Todo::new();
                        println!(
                            "Please enter the new name:(previous name was \"{}\")",
                            todo.name
                        );

                        input = String::new();
                        stdin().read_line(&mut input).expect("failed to read line");
                        new_todo.name = input.trim().to_string();
                        clearscreen::clear().expect("failed to clear screen");

                        println!("Please enter the new status: (Pending, Doing, Done)");

                        input = String::new();
                        stdin().read_line(&mut input).expect("failed to read line");
                        new_todo.status = TodoStatus::new(&input.trim().to_string());
                        clearscreen::clear().expect("failed to clear screen");

                        update_todo(id, &new_todo.name, new_todo.status);

                        println!("Updated todo: ");
                        println!("ID: {}", id);
                        println!("Name: {}", new_todo.name);
                        println!("Status: {}", input);

                        println!("Please hit the enter to exit to home!");
                    }
                    None => {
                        println!("Todo not found!");
                        println!("Please try again:");
                        stdin().read_line(&mut input).expect("failed to read line");
                    }
                }

                stdin().read_line(&mut input).expect("failed to read line");

                break;
            }
            Err(_) => {
                println!("Invalid id");
                println!("Please try again:");
                stdin().read_line(&mut input).expect("failed to read line");
            }
        }
    }
}

pub fn delete_todo_state() {
    loop {
        clearscreen::clear().expect("failed to clear screen");

        println!("Please enter the id of todo:");

        let mut input = String::new();

        stdin().read_line(&mut input).expect("failed to read line");

        let id: Result<u64, _> = input.trim().parse();

        match id {
            Ok(id) => {
                delete_todo(id);

                println!("Please hit the enter to exit to home!");
                stdin().read_line(&mut input).expect("failed to read line");

                break;
            }
            Err(_) => {
                println!("Invalid id");
                println!("Please try again:");
                stdin().read_line(&mut input).expect("failed to read line");
            }
        }
    }
}
