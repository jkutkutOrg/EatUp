use rocket::{State, get, post, patch};
use rocket::http::{Status};
use tokio_postgres::{Client};
use rocket::serde::json::Json;
use crate::tools::UuidWrapper;
use super::SessionQuery;
use crate::db;

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
) -> Result<Json<db::SessionUuid>, db::SessionError> {
    match db::create_session(db, table_id).await {
        Err(e) => Err(e),
        Ok(new_session) => Ok(Json(new_session))
    }
}

#[patch("/session/<session_id>/end")]
pub(super) async fn end_session(
    db: &State<Client>,
    session_id: UuidWrapper
) -> Result<Json<&'static str>, Status> {
    match db::end_session(db, session_id).await {
        Err(e) => Err(e),
        Ok(_) => Ok(Json("Session ended"))
    }
}
