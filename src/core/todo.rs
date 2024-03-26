use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
#[derive(PartialEq)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub done: bool
}
