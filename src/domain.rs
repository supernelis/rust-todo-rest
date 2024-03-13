use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
#[derive(PartialEq)]
pub struct Todo {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) done: bool
}
