use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub done: bool
}
