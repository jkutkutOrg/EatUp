use rocket::{State, get};
use rocket::http::{Status};
use tokio_postgres::{Client};

use crate::api::ProductQuery;
use crate::db;

#[get("/products?<filters..>")]
pub(super) async fn products(
    db: &State<Client>,
    filters: ProductQuery
) -> Result<String, Status> {
    db::get_products(db, filters).await
}
