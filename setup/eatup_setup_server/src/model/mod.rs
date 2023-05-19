use std::process::Command;
use rocket::FromForm;
use serde::{Serialize, Deserialize};

mod microservice_state;
mod microservice;

pub use microservice_state::MicroserviceState;
pub use microservice::Microservice;
