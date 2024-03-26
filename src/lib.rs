#[macro_use]
extern crate rocket;

pub use crate::controllers::rocket;

mod core;

pub use crate::core::Todo;
pub mod controllers;