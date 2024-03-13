use rocket::http::{ContentType, Status};
use rocket::local::blocking::{Client, LocalResponse};
use test_context::{test_context, TestContext};
use rust_todo_rest::rocket;

#[test_context(TodoApp)]
#[test]
fn hello_world(todo_app: &mut TodoApp) {
    let response = todo_app.get("/");

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Hello, world!");
}

struct TodoApp {
    client: Client,
}

impl TodoApp {
    pub fn patch<'a>(&'a self, uri: &'a str, body: &str) -> LocalResponse {
        self.client
            .patch(uri)
            .header(ContentType::JSON)
            .body(body)
            .dispatch()
    }
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
        self.client
            .post(uri)
            .header(ContentType::JSON)
            .body(body)
            .dispatch()
    }

    fn put<'a>(&'a self, uri: &'a str, body: &'a str) -> LocalResponse {
        self.client
            .put(uri)
            .header(ContentType::JSON)
            .body(body)
            .dispatch()
    }

    pub fn delete<'a>(&'a self, uri: &'a str) -> LocalResponse {
        self.client.delete(uri).dispatch()
    }
}