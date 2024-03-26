#[macro_use]
extern crate rocket;

use crate::controllers::rocket;

mod core;
mod controllers;

#[launch]
fn app() -> _ {
    rocket()
}