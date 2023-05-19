use rocket::{Route, routes};
use rocket::{get, post, patch};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::request::FromParam;

use super::error::InvalidAPI;
use crate::cmd;

mod microservices;

// TODO refactor into modules

#[get("/status")]
async fn get_status() -> Result<Json<&'static str>, Status> {
    Err(Status::NotImplemented) // TODO
}

#[post("/create")]
async fn create() -> Result<Status, Status> {
    Err(Status::NotImplemented) // TODO
}

#[post("/install")]
async fn install() -> Result<Status, Status> {
    Err(Status::NotImplemented) // TODO
}

#[post("/uninstall")]
async fn uninstall() -> Result<Status, Status> {
    Err(Status::NotImplemented) // TODO
}


#[patch("/create/script")]
async fn create_script(
    // TODO
) -> Result<Status, Status> {
    Err(Status::NotImplemented) // TODO
}


#[derive(Debug)]
enum MicroserviceAction {
    Start,
    Stop,
}

impl FromParam<'_> for MicroserviceAction {
    type Error = InvalidAPI;

    fn from_param(param: &'_ str) -> Result<Self, Self::Error> {
        match param {
            "start" => Ok(MicroserviceAction::Start),
            "stop" => Ok(MicroserviceAction::Stop),
            _ => Err(InvalidAPI::new("Invalid microservice action".to_string()))
        }
    }
}

#[post("/microservices/<_action>/<_name>")]
async fn microservice_action(
    _action: MicroserviceAction,
    _name: String,
) -> Result<Status, Status> {
    Err(Status::NotImplemented) // TODO
}

pub fn get_all_routes() -> Vec<Route> {
    routes![
        get_status,
        create,
        install,
        uninstall,
        create_script,
        microservices::get_all_microservices,
        microservice_action,
    ]
}