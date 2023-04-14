use tokio_postgres::{Client};
use rocket::{State, get, post};
use rocket::serde::json::Json;
use rocket::http::Status;
use crate::tools::UuidWrapper;
use crate::db;

#[get("/orders/<session_id>")]
pub(super) async fn orders(
    db: &State<Client>,
    session_id: UuidWrapper
) -> Result<Json<Vec<db::Order>>, Status> {
    match db::get_orders(db, session_id).await {
        Err(e) => Err(e),
        Ok(orders) => Ok(Json(orders))
    }
}

// #[post("/orders/<session_id>")]
// pub(super) fn create_order(session_id: u32) -> Result<String, Status> {
#[post("/orders/<_session_id>")]
pub(super) fn create_order(_session_id: u32) -> Result<String, Status> {
    return Err(Status::NotImplemented);
}