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
mod create_script;

pub fn get_all_routes() -> Vec<Route> {
    routes![
        status::get_status,
        status::create,
        status::install,
        status::uninstall,
        create_script::create_script,
        microservices::get_all_microservices,
        microservices_action::microservice_action
    ]
}