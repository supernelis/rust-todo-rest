#[macro_use]
extern crate rocket;

use rocket::http::{Header, Status};
use rocket::{Request, response, Response};
use rocket::response::Responder;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/tasks")]
fn add_task() -> TodoCreatedResponse {
    TodoCreatedResponse {
        id: "1".to_string()
    }
}

struct TodoCreatedResponse {
    id: String
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for TodoCreatedResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .header(Header::new("Location", format!("/tasks/{}",self.id)))
            .status(Status::Created)
            .ok()
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, add_task])
}

#[cfg(test)]
mod test {
    use rocket::http::{ContentType, Status};
    use rocket::http::hyper::header::LOCATION;
    use rocket::local::blocking::Client;

    use super::rocket;

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }

    #[test]
    fn test_add_task() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        
        let response = client
            .post("/tasks/")
            .header(ContentType::JSON)
            .body(r##"
            {
                "title": "a title"
            }
            "##)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
        assert_eq!(response.headers().get_one(LOCATION.as_str()).unwrap(), "/tasks/1");
    }
}