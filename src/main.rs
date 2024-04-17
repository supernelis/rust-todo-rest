use rocket::launch;
use rust_todo_rest::create_todo_app;

#[launch]
fn app() -> _ {
    env_logger::init();
    create_todo_app()
}