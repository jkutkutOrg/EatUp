// use serde_json::json;
use std::process::Command;

#[derive(Debug)]
pub enum MicroserviceState {
    Created,
    Running,
    Restarting,
    Exited,
    Paused,
    Dead,
    NotFound,
    Ndef
}

impl From<&str> for MicroserviceState {
    fn from(s: &str) -> Self {
        match s {
            "created" => MicroserviceState::Created,
            "running" => MicroserviceState::Running,
            "restarting" => MicroserviceState::Restarting,
            "exited" => MicroserviceState::Exited,
            "paused" => MicroserviceState::Paused,
            "dead" => MicroserviceState::Dead,
            _ => MicroserviceState::Ndef
        }
    }
}

impl MicroserviceState {
    fn from_microservice_id(id: &str) -> Self {
        let mut cmd = Command::new("docker");
        cmd.args(&["inspect", "-f", "{{.State.Status}}", &id]);
        let output = cmd.output().expect("failed to inspect container");
        let state = String::from_utf8(output.stdout).unwrap();
        MicroserviceState::from(state.trim())
    }
}

#[derive(Debug)]
pub struct Microservice {
    name: String,
    state: MicroserviceState,
    requires: Vec<String>
}

impl Microservice {
    fn from_info_state(info: &MicroserviceInfo, state: MicroserviceState) -> Self {
        Microservice {
            name: info.name.to_string(),
            state: state,
            requires: info.requires.iter().map(|s| s.to_string()).collect()
        }
    }

    fn from_info(info: MicroserviceInfo) -> Self {
        Microservice::from_info_state(
            &info,
            match Microservice::get_microservice_id(info.name) {
                None => MicroserviceState::NotFound,
                Some(id) => MicroserviceState::from_microservice_id(&id)
            }
        )
    }

    fn get_microservice_id(name: &str) -> Option<String> {
        let mut cmd = Command::new("docker");
        cmd.args(&["ps", "-qaf", &format!("name={name}")]);
        let output = cmd.output().expect("failed to ps container by name");
        let id = String::from_utf8(output.stdout).unwrap().trim().to_string();
        match id.len() {
            0 => None,
            _ => Some(id)
        }
    }
}

#[derive(Debug, Clone)]
pub struct MicroserviceInfo<'a> {
    name: &'a str,
    requires: Vec<&'a str>
}

struct MicroserviceInfoCollection<'a> {
    collection: Vec<MicroserviceInfo<'a>>
}

impl<'a> MicroserviceInfoCollection<'a> {
    fn new(collection: [MicroserviceInfo<'a>; 2]) -> Self {
        MicroserviceInfoCollection {
            collection: collection.to_vec()
        }
    }

    fn get_by_name(&self, name: &str) -> Option<MicroserviceInfo> {
        for info in &self.collection {
            if info.name == name {
                return Some(info.clone());
            }
        }
        None
    }

    fn all(&self) -> Vec<MicroserviceInfo> {
        self.collection.clone()
    }
}

use lazy_static::lazy_static;
lazy_static! {
    static ref MICROSERVICES_INFO: MicroserviceInfoCollection<'static> = MicroserviceInfoCollection::new(
        [
            MicroserviceInfo {
                name: "eatup_db",
                requires: vec![]
            },
            MicroserviceInfo {
                name: "eatup_server",
                requires: vec!["eatup_db"]
            }
        ]
    );
}

// -----------------------------------------------

pub fn get_microservice(name: String) -> Result<Microservice, String> {
    match MICROSERVICES_INFO.get_by_name(&name) {
        Some(micro) => Ok(Microservice::from_info(micro)),
        None => Err(format!("Microservice {} not found", name))
    }
}

pub fn get_microservices(names: Vec<String>) -> Vec<Result<Microservice, String>> {
    let mut microservices = vec![];
    for name in names {
        microservices.push(get_microservice(name));
    }
    microservices
}

pub fn get_all_microservices() -> Vec<Microservice> {
    let mut microservices = vec![];
    for info in MICROSERVICES_INFO.all() {
        microservices.push(Microservice::from_info(info));
    }
    microservices
}