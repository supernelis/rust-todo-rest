use std::collections::HashMap;
use std::sync::Mutex;
use mockall::automock;

use rocket::{Build, Rocket, Route};
use rocket::routes;

use crate::core::Todo;

mod todo_created_response;
mod todo_create;
mod todo_update;
mod todo_controller;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn create_todo_app() -> Rocket<Build> {
    let todos: Mutex<HashMap<String, Todo>> = Mutex::new(HashMap::new());
    rocket::build()
        .manage(todos)
        .mount("/", todo_routes())
}

fn todo_routes() -> Vec<Route> {
    [routes![index], todo_controller::routes()].concat()
}

#[automock]
pub trait Reporter{
    fn report_todo_created(&self);
}