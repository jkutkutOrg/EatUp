use rocket::{State, get, post, patch};
use rocket::http::{Status};
use tokio_postgres::{Client};
use super::SessionQuery;
use crate::db;
use rocket::serde::json::Json;

#[get("/sessions?<filters..>")]
pub(super) async fn sessions(
    db: &State<Client>,
    filters: SessionQuery
) -> Result<Json<Vec<db::Session>>, Status> {
    match db::get_sessions(db, filters).await {
        Err(e) => Err(e),
        Ok(products) => Ok(Json(products))
    }
}

// #[post("/sessions/<table_id>")]
// pub(super) fn create_session(table_id: u32) -> Result<String, Status> {
#[post("/sessions/<_table_id>")]
pub(super) fn create_session(_table_id: u32) -> Result<String, Status> {
    return Err(Status::NotImplemented);
}

// #[patch("/sessions/<session_id>/end")]
// pub(super) fn end_session(session_id: u32) -> Result<String, Status> {
#[patch("/sessions/<_session_id>/end")]
pub(super) fn end_session(_session_id: u32) -> Result<String, Status> {
    return Err(Status::NotImplemented);
}
