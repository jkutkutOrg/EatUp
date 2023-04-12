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

#[post("/session/<table_id>")]
pub(super) async fn create_session(
    db: &State<Client>,
    table_id: String
) -> Result<Json<db::SessionMap>, Status> {
    match db::create_session(db, table_id).await {
        Err(e) => Err(e),
        Ok(new_session) => Ok(Json(new_session))
    }
}

// #[patch("/session/<session_id>/end")]
// pub(super) fn end_session(session_id: String) -> Result<String, Status> {
#[patch("/session/<_session_id>/end")]
pub(super) fn end_session(_session_id: String) -> Result<String, Status> {
    return Err(Status::NotImplemented);
}
