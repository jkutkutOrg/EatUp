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
            match get_microservice_id(info.name) {
                None => MicroserviceState::NotFound,
                Some(id) => MicroserviceState::from_microservice_id(&id)
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct MicroserviceInfo<'a> {
    name: &'a str,
    requires: Vec<&'a str>
}

impl MicroserviceInfo<'_> {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "eatup_db" => Some(
                MicroserviceInfo {
                    name: "eatup_db",
                    requires: vec![]
                }
            ),
            "eatup_server" => Some(
                MicroserviceInfo {
                    name: "eatup_server",
                    requires: vec!["eatup_db"]
                }
            ),
            _ => None
        }
    }
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

// -----------------------------------------------

pub fn get_microservice(name: String) -> Result<Microservice, String> {
    match MicroserviceInfo::from_name(&name) {
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