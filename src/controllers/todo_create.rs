use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoCreate {
    pub(crate) title: String,
}
