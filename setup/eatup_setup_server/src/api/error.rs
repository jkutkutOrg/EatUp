use rocket::{catch, Responder};
use rocket::serde::json::Json;

#[catch(404)]
pub fn not_found() -> Json<&'static str> {
    Json("404 not found. Are you sure the request is valid?")
}

#[catch(501)]
pub fn not_implemented() -> Json<&'static str> {
    Json("Ups, this feature is not implemented yet.")
}

#[catch(500)]
pub fn internal_server_error() -> Json<&'static str> {
    Json("Well, this is embarrassing. Something went wrong on our side. Turns out, rust can fail too.")
}

#[derive(Debug, Responder)]
#[response(status = 409, content_type = "json")]
pub struct InvalidAPI {
    message: String
}

impl InvalidAPI {
    pub fn new(message: String) -> InvalidAPI {
        InvalidAPI { message }
    }
}