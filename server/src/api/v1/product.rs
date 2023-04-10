use rocket::{State, get};
use rocket::http::{Status};
use tokio_postgres::{Client};
use super::ProductQuery;
use crate::db;
use rocket::serde::json::Json;

#[get("/products?<filters..>")]
pub(super) async fn products(
    db: &State<Client>,
    filters: ProductQuery
) -> Result<Json<Vec<db::Product>>, Status> {
    match db::get_products(db, filters).await {
        Err(e) => Err(e),
        Ok(products) => Ok(Json(products))
    }
}