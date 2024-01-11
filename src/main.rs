#[macro_use]
extern crate rocket;

use rocket::figment::value::Empty;
use rocket::http::{Header, Status};
use rocket::response;
use rocket::response::{Responder, status};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/tasks")]
fn add_task() -> MyResponder {
    MyResponder {
        status: Status::Created,
        header: Header::new("Location","/tasks/1")
    }
}

#[derive(Responder)]
struct MyResponder {
    status: Status,
    header: Header<'static>,
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