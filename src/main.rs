#[macro_use]
extern crate rocket;

use crate::controllers::rocket;

mod core;
mod controllers;

#[launch]
fn app() -> _ {
    rocket()
}

#[cfg(test)]
mod test {
    use rocket::http::{ContentType, Status};
    use rocket::http::hyper::header::LOCATION;
    use rocket::local::blocking::{Client, LocalResponse};
    use test_context::{test_context, TestContext};
    use crate::controllers::rocket;

    use crate::core::Todo;

    use super::app;

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
    fn test_update_task(todo_app: &mut TodoApp) {
        let response = todo_app.put("/tasks/1", r##"
            {
                "title": "another title"
            }
            "##);

        assert_eq!(response.status(), Status::Ok);
    }

    #[test_context(TodoApp)]
    #[test]
    fn test_get_task(todo_app: &mut TodoApp) {
        let create_task_response = todo_app.post("/tasks/", r##"
            {
                "title": "new title"
            }
            "##);

        let get_task_response = todo_app.get(create_task_response.extract_location());
        assert_eq!(get_task_response.status(), Status::Ok);
        let todo = get_task_response.extract_todo();
        assert_eq!(todo.id, "1");
        assert_eq!(todo.title, "new title");
        assert_eq!(todo.done, false)
    }


    #[test_context(TodoApp)]
    #[test]
    fn test_get_task_fails_with_404_when_getting_non_existent_task(todo_app: &mut TodoApp) {
        let response = todo_app.get("/tasks/123");
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test_context(TodoApp)]
    #[test]
    fn test_delete(todo_app: &mut TodoApp) {
        let response = todo_app.post("/tasks/", r##"
            {
                "title": "new title"
            }
            "##);

        let location = response.extract_location();

        let delete_response = todo_app.delete(location);
        assert_eq!(delete_response.status(), Status::Accepted);

        let get_response = todo_app.get(location);
        assert_eq!(get_response.status(), Status::NotFound);
    }

    #[test_context(TodoApp)]
    #[test]
    fn test_delete_a_non_existing_task(todo_app: &mut TodoApp) {
        let delete_response = todo_app.delete("/tasks/1");
        assert_eq!(delete_response.status(), Status::NotFound);
    }

    #[test_context(TodoApp)]
    #[test]
    fn test_complete_a_task(todo_app: &mut TodoApp) {
        let response = todo_app.post("/tasks/", r##"
            {
                "title": "new title"
            }
            "##);

        let location = response.extract_location();
        let response = todo_app.patch(location, r##"
            {
                "done": true
            }
            "##);

        assert_eq!(response.status(), Status::Accepted);

        let get_task_response = todo_app.get(location);
        let todo = get_task_response.extract_todo();
        assert_eq!(todo.done, true)
    }

    #[test_context(TodoApp)]
    #[test]
    fn test_mark_as_complete_a_nonexistent_task(todo_app: &mut TodoApp) {
        let response = todo_app.patch("/tasks/1", r##"
            {
                "done": true
            }
            "##);

        assert_eq!(response.status(), Status::NotFound);
    }

    trait ExtractResponses {
        fn extract_location(&self) -> &str;
        fn extract_todo(self) -> Todo;
    }

    impl ExtractResponses for LocalResponse<'_> {
        fn extract_location(&self) -> &str {
            self.headers()
                .get_one("Location")
                .unwrap()
        }

        fn extract_todo(self) -> Todo {
            self.into_json::<Todo>().unwrap()
        }
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
}