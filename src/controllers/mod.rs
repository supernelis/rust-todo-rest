mod todo_created_response;

use rocket::serde::json::Json;
use rocket::{Route, State};
use std::sync::Mutex;
use std::collections::HashMap;
use rocket::http::Status;
use rocket::serde::Deserialize;
pub use todo_created_response::TodoCreatedResponse;
use crate::core::Todo;

pub fn todo_routes() -> Vec<Route> {
    routes![index, add_task, update_task, patch_task, get_task, delete_task]
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct TodoCreate {
    title: String,
}

#[post("/tasks", data = "<todo>")]
fn add_task(todo: Json<TodoCreate>, todos: &State<Mutex<HashMap<String, Todo>>>) -> TodoCreatedResponse {
    let mut todos_map = todos.lock().unwrap();
    let todo_index = todos_map.len() + 1;
    todos_map.insert(todo_index.to_string(), Todo {
        id: todo_index.to_string(),
        title: todo.title.clone(),
        done: false,
    });
    TodoCreatedResponse {
        id: todo_index.to_string()
    }
}

#[put("/tasks/<_id>", data = "<_input>")]
fn update_task(_id: String, _input: Json<TodoUpdate>) -> Status {
    Status::Ok
}

#[patch("/tasks/<id>", data = "<input>")]
fn patch_task(id: String, input: Json<TodoUpdate>, todos: &State<Mutex<HashMap<String, Todo>>>) -> Status {
    let mut todos_map = todos.lock().unwrap();

    match todos_map.get_mut(&id.to_string()) {
        Some(todo) => {
            todo.done = input.done.unwrap();

            Status::Accepted
        }
        None => {
            Status::NotFound
        }
    }
}

#[get("/tasks/<id>")]
fn get_task(id: &str, todos: &State<Mutex<HashMap<String, Todo>>>) -> Option<Json<Todo>> {
    let todos_map = todos.lock().unwrap();
    todos_map.get(&id.to_string())
        .map(|todo| Json(todo.clone()))
}

#[delete("/tasks/<id>")]
fn delete_task(id: &str, todos: &State<Mutex<HashMap<String, Todo>>>) -> Status {
    let mut todos_map = todos.lock().unwrap();
    let remove_status = todos_map.remove(id);
    if remove_status == None {
        return Status::NotFound;
    }
    Status::Accepted
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct TodoUpdate {
    done: Option<bool>,
}
