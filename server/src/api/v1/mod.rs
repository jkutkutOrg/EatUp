use tokio_postgres::{Client};
use rocket::{routes, State, get, post, patch};
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::db;
use crate::tools::{UuidWrapper, route_error::InvalidAPI};

mod model;
mod product;
mod session;
mod order;

pub use model::*;

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
