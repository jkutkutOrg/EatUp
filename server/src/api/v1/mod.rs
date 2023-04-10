use rocket::routes;
use rocket::FromForm;
use rocket::serde::Deserialize;
use crate::tools::UuidWrapper;

#[derive(Debug, Deserialize, FromForm)]
pub struct ProductQuery {
    #[field(name = "category")]
    pub categories: Vec<String>,
    #[field(name = "allergy")]
    pub allergies: Vec<String>,
}

#[derive(Debug, Deserialize, FromForm)]
pub struct SessionQuery {
    pub table_id: Option<UuidWrapper>,
    pub in_progress: bool
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