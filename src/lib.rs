#[macro_use]
extern crate rocket;

mod controllers;
pub use crate::controllers::rocket;

mod core;
pub use crate::core::Todo;
