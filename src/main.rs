mod actions;
mod todo;
mod todo_status;

use crate::actions::{create_todo, get_single_todo, get_todos};

fn main() {
    let todos = get_todos().unwrap_or_default();

    // for todo in todos {
    //     println!("ID: {}", todo.id);
    //     println!("Name: {}", todo.name);
    //     println!("Status: {}", todo.status);
    //     println!("-------------------------")
    // }

    // let todo = get_single_todo(5);

    // if let Some(todo) = todo {
    //     println!("ID: {}", todo.id);
    //     println!("Name: {}", todo.name);
    //     println!("Status: {}", todo.status);
    // }

    create_todo("Learning rust", todo_status::TodoStatus::Pending);
}
