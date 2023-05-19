use crate::model::Microservice;

const MICROSERVICES: [&'static str; 2] = [
    "eatup_db",
    "eatup_server"
];

pub fn get_all_microservices() -> Vec<Microservice> {
    let mut microservices = vec![];
    for m in MICROSERVICES.iter() {
        microservices.push(Microservice::new_by_name(m.to_string()));
    }
    microservices
}