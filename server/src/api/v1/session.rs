use rocket::{State, get, post, patch};
use tokio_postgres::Client;

use rocket::http::{Status};

#[get("/sessions")]
pub(super) async fn sessions(db: &State<Client>) -> Result<String, Status> {
    let stmt = db.query("SELECT name FROM product", &[]).await.unwrap();
    for row in stmt {
        let name: String = row.get(0);
        println!("name: {}", name);
    }
    return Err(Status::NotImplemented);
}

#[post("/sessions/<table_id>")]
pub(super) fn create_session(table_id: u32) -> Result<String, Status> {
    return Err(Status::NotImplemented);
}

#[patch("/sessions/<session_id>/end")]
pub(super) fn end_session(session_id: u32) -> Result<String, Status> {
    return Err(Status::NotImplemented);
}
