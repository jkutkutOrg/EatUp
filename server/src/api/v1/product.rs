use rocket::{State, get};
use rocket::http::{Status};
use tokio_postgres::{Client};
use serde_json;

use crate::api::ProductQuery;
use crate::db;

#[get("/products?<filters..>")]
pub(super) fn products(
    db: &State<Client>,
    filters: ProductQuery
) -> Result<String, Status> {

    match db::get_products(db, filters) {
        Err(e) => Err(e),
        Ok(products) => Ok(serde_json::to_string(&products).unwrap())
    }
}
