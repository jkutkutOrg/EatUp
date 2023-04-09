use rocket::{/*State,*/ get, post};
use rocket::http::Status;

#[get("/orders/<session_id>")]
pub(super) fn orders(session_id: u32) -> Result<String, Status> {
    return Err(Status::NotImplemented);
}

#[post("/orders/<session_id>")]
pub(super) fn create_order(session_id: u32) -> Result<String, Status> {
    return Err(Status::NotImplemented);
}