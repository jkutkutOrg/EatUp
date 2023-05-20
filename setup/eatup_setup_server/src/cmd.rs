use std::path::Path;

use crate::model::*;
use crate::MICROSERVICES;
use crate::api::error::InvalidAPI;

pub fn get_all_microservices() -> Vec<Microservice> {
    let mut microservices = vec![];
    for m in MICROSERVICES.iter() {
        microservices.push(Microservice::by_name(m.to_string()));
    }
    microservices
}

pub fn microservice_action(
    action: MicroserviceAction,
    name: String
) -> Option<InvalidAPI> {
    match MICROSERVICES.iter().find(|&m| m == &name) {
        None => return Some(InvalidAPI::new(
            "This container does not exist or belong to this project.".to_string()
        )),
        _ => ()
    }
    match Microservice::by_name(name.to_string()).do_action(action) {
        Some(e) => Some(InvalidAPI::new(e)),
        None => None
    }
}

pub fn get_status() -> ProjectState {
    let public_dir_exists = Path::new("/installation/public").exists();
    if !public_dir_exists {
        return ProjectState::NotCreated;
    }
    let db_container = Microservice::by_name("eatup_db".to_string());
    if db_container.get_state() == MicroserviceState::NotFound {
        return ProjectState::Created;
    }
    ProjectState::Installed
}