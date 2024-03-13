use rocket::{Request, response, Response};
use rocket::http::{Header, Status};
use rocket::response::Responder;

pub struct TodoCreatedResponse {
    pub(crate) id: String,
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
