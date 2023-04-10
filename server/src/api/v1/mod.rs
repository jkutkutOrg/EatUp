use rocket::routes;
// use rocket::serde::{Deserialize};

// use tokio_postgres::Client;

use rocket::FromForm;
use rocket::serde::Deserialize;

#[derive(Debug, Deserialize, FromForm)]
pub struct ProductQuery {
    #[field(name = "category")]
    pub categories: Vec<String>,
    #[field(name = "allergy")]
    pub allergies: Vec<String>,
}

mod product;
mod session;
mod order;

pub fn get_all_routes() -> Vec<rocket::Route> {
    routes![
        product::products,
        session::sessions,
        session::create_session,
        session::end_session,
        order::orders,
        order::create_order
    ]
}