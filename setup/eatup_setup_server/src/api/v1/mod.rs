use rocket::{Route, routes};
use rocket::{get, post, patch};
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::cmd;
// use super::error::InvalidAPI;

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


#[post("/microservices/start/<_name>")]
async fn start_microservice(_name: String) -> Result<Status, Status> {
    Err(Status::NotImplemented) // TODO
}

#[post("/microservices/stop/<_name>")]
async fn stop_microservice(_name: String) -> Result<Status, Status> {
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
        start_microservice,
        stop_microservice,
    ]
}