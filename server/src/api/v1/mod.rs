use tokio_postgres::{Client};
use rocket::{routes, State, get, post, patch};
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::db;
use crate::tools::UuidWrapper;
use crate::route_tools::InvalidAPI;

pub use model::*;

mod model {
    use rocket::FromForm;
    use rocket::serde::Deserialize;
    use crate::db::model::Product;

    #[derive(Deserialize, FromForm)]
    pub struct ProductQuery {
        #[field(name = "category")]
        pub categories: Vec<String>,
        #[field(name = "allergy")]
        pub allergies: Vec<String>,
    }

    #[derive(Deserialize, FromForm)]
    pub struct SessionQuery {
        #[field(name = "table_id")]
        pub table_ids: Vec<String>,
        pub in_progress: Option<bool>,
    }

    #[derive(Deserialize)]
    pub struct OrderQuery {
        pub products: Vec<ProductOrderQuery>
    }

    #[derive(Deserialize)]
    pub struct ProductOrderQuery {
        pub quantity: i32,
        pub product: Product
    }
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