#[macro_use]
extern crate rocket;

mod controllers;
pub use crate::controllers::create_todo_app;

mod core;
