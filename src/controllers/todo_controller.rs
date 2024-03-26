use std::collections::HashMap;
use std::sync::Mutex;

use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::controllers::todo_create::TodoCreate;
pub use crate::controllers::todo_created_response::TodoCreatedResponse;
use crate::controllers::todo_update::TodoUpdate;
use crate::core::Todo;

#[post("/tasks", data = "<todo>")]
pub fn add_task(todo: Json<TodoCreate>, todos: &State<Mutex<HashMap<String, Todo>>>) -> TodoCreatedResponse {
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
pub fn update_task(_id: String, _input: Json<TodoUpdate>) -> Status {
    Status::Ok
}

#[patch("/tasks/<id>", data = "<input>")]
pub fn patch_task(id: String, input: Json<TodoUpdate>, todos: &State<Mutex<HashMap<String, Todo>>>) -> Status {
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
pub fn get_task(id: &str, todos: &State<Mutex<HashMap<String, Todo>>>) -> Option<Json<Todo>> {
    let todos_map = todos.lock().unwrap();
    todos_map.get(&id.to_string())
        .map(|todo| Json(todo.clone()))
}

#[delete("/tasks/<id>")]
pub fn delete_task(id: &str, todos: &State<Mutex<HashMap<String, Todo>>>) -> Status {
    let mut todos_map = todos.lock().unwrap();
    let remove_status = todos_map.remove(id);
    if remove_status == None {
        return Status::NotFound;
    }
    Status::Accepted
}