use crate::model::Microservice;
use crate::MICROSERVICES;
use crate::model::MicroserviceAction;
use crate::api::error::InvalidAPI;

pub fn get_all_microservices() -> Vec<Microservice> {
    let mut microservices = vec![];
    for m in MICROSERVICES.iter() {
        microservices.push(Microservice::new_by_name(m.to_string()));
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
    match Microservice::new_by_name(name.to_string()).do_action(action) {
        Some(e) => Some(InvalidAPI::new(e)),
        None => None
    }
}