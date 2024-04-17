#[macro_use]
extern crate rocket;

mod controllers;
pub use crate::controllers::create_todo_app;
pub use crate::controllers::MockReporter;
pub use crate::controllers::ConsoleReporter;

mod core;
