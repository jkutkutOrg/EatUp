use super::*;

#[derive(Debug, Serialize)]
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
    pub fn new_by_id(id: &str) -> Self {
        let mut cmd = Command::new("docker");
        cmd.args(&["inspect", "-f", "{{.State.Status}}", &id]);
        let output = cmd.output().expect("failed to inspect container");
        let state = String::from_utf8(output.stdout).unwrap();
        MicroserviceState::from(state.trim())
    }
}