use crate::{todo::Todo, todo_status::TodoStatus};
use std::fs;

pub fn get_todos() -> Result<Vec<Todo>, String> {
    let data = fs::read_to_string("todos.json").unwrap_or_else(|_| "{\"todos\": []}".to_string());

    let parsed = json::parse(&data).map_err(|e| format!("JSON Parse Error: {}", e))?;

    let mut todos_vec = Vec::new();

    for item in parsed["todos"].members() {
        todos_vec.push(Todo {
            id: item["id"].as_u64().unwrap_or(0),
            name: item["name"].as_str().unwrap_or("Unknown").to_string(),
            status: TodoStatus::new(item["status"].as_str().unwrap_or("Pending")),
        });
    }

    Ok(todos_vec)
}

pub fn get_single_todo(id: u64) -> Option<Todo> {
    let todos = get_todos().unwrap_or_default();
    todos.into_iter().find(|todo| todo.id == id)
}

pub fn create_todo(name: &str, status: TodoStatus) {
    let mut todos = get_todos().unwrap_or_default();

    let new_id = todos.last().map_or(1, |last| last.id + 1);

    todos.push(Todo {
        id: new_id,
        name: name.to_string(),
        status: status,
    });

    let mut json_array = json::Array::new();
    for t in todos {
        json_array.push(json::object! {
            "id": t.id,
            "name": t.name,
            "status": t.status.to_string()
        });
    }

    let final_json = json::object! { "todos": json_array };

    if let Err(e) = fs::write("todos.json", final_json.pretty(4)) {
        eprintln!("Failed to save todo: {}", e);
    } else {
        println!("Successfully created todo: {}", name);
    }
}

pub fn update_todo(id: u64, name: &str, status: TodoStatus) {
    let mut todos = get_todos().unwrap_or_default();

    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
        todo.name = name.to_string();
        todo.status = status;
    }

    let mut json_array = json::Array::new();
    for t in todos {
        json_array.push(json::object! {
            "id": t.id,
            "name": t.name,
            "status": t.status.to_string()
        });
    }

    let final_json = json::object! { "todos": json_array };

    if let Err(e) = fs::write("todos.json", final_json.pretty(4)) {
        eprintln!("Failed to save todo: {}", e);
    } else {
        println!("Successfully updated todo: {}", name);
    }
}

pub fn delete_todo(id: u64) {
    let mut todos = get_todos().unwrap_or_default();

    if let Some(index) = todos.iter().position(|todo| todo.id == id) {
        todos.remove(index);
    }

    let mut json_array = json::Array::new();
    for t in todos {
        json_array.push(json::object! {
            "id": t.id,
            "name": t.name,
            "status": t.status.to_string()
        });
    }

    let final_json = json::object! { "todos": json_array };

    if let Err(e) = fs::write("todos.json", final_json.pretty(4)) {
        eprintln!("Failed to save todo: {}", e);
    } else {
        println!("Successfully deleted todo: {}", id);
    }
}
