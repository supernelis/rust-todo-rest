use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoUpdate {
    pub(crate) done: Option<bool>,
}
