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

#[derive(Debug)]
pub struct Microservice {
    name: String,
    state: MicroserviceState,
    requires: Vec<String>
}

impl Microservice {
    pub fn from_info(info: MicroserviceInfo, state: MicroserviceState) -> Self {
        Microservice {
            name: info.name.to_string(),
            state: state,
            requires: info.requires.iter().map(|s| s.to_string()).collect()
        }
    }
}

#[derive(Debug, Clone)]
pub struct MicroserviceInfo<'a> {
    name: &'a str,
    requires: Vec<&'a str>
}

fn find_microservice(name: &String) -> Result<MicroserviceInfo, String> {
    match name.as_str() {
        "eatup_db" => Ok(MicroserviceInfo {
            name: "eatup_db",
            requires: vec![]
        }),
        "eatup_server" => Ok(MicroserviceInfo {
            name: "eatup_server",
            requires: vec!["eatup_db"]
        }),
        _ => Err("Microservice not found".to_string()),
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

fn get_microservice_state(id: String) -> MicroserviceState {
    let mut cmd = Command::new("docker");
    cmd.args(&["inspect", "-f", "{{.State.Status}}", &id]);
    let output = cmd.output().expect("failed to inspect container");
    let state = String::from_utf8(output.stdout).unwrap();
    MicroserviceState::from(state.trim())
}

pub async fn get_microservice(name: String) -> Result<Microservice, String> {
    let micro = match find_microservice(&name) {
        Ok(micro) => micro,
        Err(e) => return Err(e)
    };
    Ok(match get_microservice_id(&name) {
        None => Microservice::from_info(micro, MicroserviceState::NotFound),
        Some(id) => Microservice::from_info(micro, get_microservice_state(id))
    })
}