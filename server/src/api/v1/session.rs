use super::*;

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

#[get("/session_id/<simple_id>")]
pub(super) async fn session_id(
    db: &State<Client>,
    simple_id: String
) -> Result<Json<db::SessionUuid>, InvalidAPI> {
    match db::get_session_id(db, simple_id).await {
        Err(e) => Err(e),
        Ok(product) => Ok(Json(product))
    }
}

#[post("/session/<table_id>")]
pub(super) async fn create_session(
    db: &State<Client>,
    table_id: String
) -> Result<Json<db::SessionUuid>, InvalidAPI> {
    match db::create_session(db, table_id).await {
        Err(e) => Err(e),
        Ok(new_session) => Ok(Json(new_session))
    }
}

#[patch("/session/<session_id>/end")]
pub(super) async fn end_session(
    db: &State<Client>,
    session_id: UuidWrapper
) -> Result<Json<&'static str>, InvalidAPI> {
    let session_id = match session_id.get() {
        Ok(session_id) => session_id,
        Err(_) => return Err(InvalidAPI::new(ERROR_INVALID_SESSION_ID))
    };
    match db::end_session(db, session_id).await {
        Err(e) => Err(e),
        Ok(_) => Ok(Json("Session ended"))
    }
}
