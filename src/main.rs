#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::sync::Mutex;

use rocket::{Request, response, Response, State};
use rocket::http::{Header, Status};
use rocket::response::Responder;
use rocket::serde::{Deserialize, json::Json};
use serde::Serialize;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/tasks", data = "<todo>")]
fn add_task(todo: Json<TodoUpdate>, todos: &State<Mutex<HashMap<String, Todo>>>) -> TodoCreatedResponse {
    let mut todos_map = todos.lock().unwrap();
    let todo_index = todos_map.len() + 1;
    todos_map.insert(todo_index.to_string(), Todo {
        id: todo_index.to_string(),
        title: todo.title.clone(),
    });
    TodoCreatedResponse {
        id: todo_index.to_string()
    }
}

#[put("/tasks/<_id>", data = "<_input>")]
fn update_task(_id: String, _input: Json<TodoUpdate>) -> Status {
    Status::Ok
}

#[get("/tasks/<id>")]
fn get_task(id: &str, todos: &State<Mutex<HashMap<String, Todo>>>) -> Option<Json<Todo>> {
    let todos_map = todos.lock().unwrap();
    todos_map.get(&id.to_string())
        .map(|todo| Json(todo.clone()))
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct TodoUpdate {
    title: String,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Todo {
    id: String,
    title: String,
}


struct TodoCreatedResponse {
    id: String,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for TodoCreatedResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .header(Header::new("Location", format!("/tasks/{}", self.id)))
            .status(Status::Created)
            .ok()
    }
}

#[launch]
fn rocket() -> _ {
    let todos: Mutex<HashMap<String, Todo>> = Mutex::new(HashMap::new());
    rocket::build()
        .manage(todos)
        .mount("/", routes![index, add_task, update_task, get_task])
}

#[cfg(test)]
mod test {
    use rocket::http::{ContentType, Status};
    use rocket::http::hyper::header::LOCATION;
    use rocket::local::blocking::{Client, LocalResponse};
    use test_context::{test_context, TestContext};

    use super::{rocket, Todo};

    struct TodoApp {
        client: Client,
    }

    impl TestContext for TodoApp {
        fn setup() -> Self {
            Self {
                client: Client::tracked(rocket()).expect("valid rocket instance")
            }
        }
    }

    impl TodoApp {
        fn get<'a>(&'a self, uri: &'a str) -> LocalResponse {
            self.client.get(uri).dispatch()
        }

        fn post<'a>(&'a self, uri: &'a str, body: &'a str) -> LocalResponse {

            let response = self.client
                .post(uri)
                .header(ContentType::JSON)
                .body(body)
                .dispatch();
            response
        }
    }

    #[test_context(TodoApp)]
    #[test]
    fn hello_world(todo_app: &mut TodoApp) {
        let response = todo_app.get("/");

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }

    #[test_context(TodoApp)]
    #[test]
    fn test_add_task(todo_app: &mut TodoApp) {
        let response = todo_app.post("/tasks/", r##"
            {
                "title": "a title"
            }
            "##);

        assert_eq!(response.status(), Status::Created);
        assert_eq!(response.headers().get_one(LOCATION.as_str()).unwrap(), "/tasks/1");
    }

    #[test_context(TodoApp)]
    #[test]
    fn test_update_task(ctx: &mut TodoApp) {
        let response = ctx.client
            .put("/tasks/1")
            .header(ContentType::JSON)
            .body(r##"
            {
                "title": "another title"
            }
            "##)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test_context(TodoApp)]
    #[test]
    fn test_get_task(todo_app: &mut TodoApp) {
        let response = todo_app.post("/tasks/",r##"
            {
                "title": "new title"
            }
            "##);

        let response = todo_app.get(response.location());
        assert_eq!(response.status(), Status::Ok);
        let todo = response.into_json::<Todo>().unwrap();
        assert_eq!(todo.id, "1");
        assert_eq!(todo.title, "new title")
    }

    trait LocationHeader {
        fn location(&self) -> &str;
    }

    impl LocationHeader for LocalResponse<'_> {
        fn location(&self) -> &str {
            self.headers()
                .get_one("Location")
                .unwrap()
        }
    }

    #[test_context(TodoApp)]
    #[test]
    fn test_get_task_fails_with_404_when_getting_non_existent_task(todo_app: &mut TodoApp) {
        let response = todo_app.get("/tasks/123");
        assert_eq!(response.status(), Status::NotFound);
    }
}