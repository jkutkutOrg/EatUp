use rocket::{Route, routes};
use rocket::{get, post, patch};
use rocket::http::Status;
use rocket::serde::json::Json;

use super::error::InvalidAPI;
use crate::cmd;
use crate::model::*;

mod microservices_action;
mod microservices;
mod status;

// TODO refactor into modules

#[patch("/create/script")]
async fn create_script(
    // TODO
) -> Result<Status, Status> {
    Err(Status::NotImplemented) // TODO
}

pub fn get_all_routes() -> Vec<Route> {
    routes![
        status::get_status,
        status::create,
        status::install,
        status::uninstall,
        create_script,
        microservices::get_all_microservices,
        microservices_action::microservice_action
    ]
}