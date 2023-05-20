use std::process::Command;
// use rocket::FromForm;
use serde::{Serialize/*, Deserialize*/};

mod microservice_action;
mod microservice_state;
mod microservice;
mod project_state;

pub use microservice_action::MicroserviceAction;
pub use microservice_state::MicroserviceState;
pub use microservice::Microservice;
pub use project_state::ProjectState;
