use rocket::launch;
use rust_todo_rest::rocket;

#[launch]
fn app() -> _ {
    rocket()
}