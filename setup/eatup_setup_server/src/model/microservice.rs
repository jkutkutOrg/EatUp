use super::*;

#[derive(Serialize)]
pub struct Microservice {
    id: Option<String>,
    name: String,
    state: MicroserviceState,
    ip: Option<String>,
    port: Option<String>,
}

impl Microservice {
    pub fn new(
        id: Option<String>,
        name: String,
        state: MicroserviceState,
        ip: Option<String>,
        port: Option<String>
    ) -> Self {
        Microservice {id, name, state, ip, port}
    }

    pub fn by_name(name: String) -> Self {
        let id = Microservice::get_microservice_id(&name);
        let state = match &id {
            Some(id) => MicroserviceState::new_by_id(&id),
            None => MicroserviceState::NotFound
        };
        let mut ip: Option<String> = None;
        let mut port: Option<String> = None;
        if state == MicroserviceState::Running {
            let full_ip = Microservice::get_ip_port(id.clone().unwrap());
            ip = Some(full_ip[0].clone());
            port = Some(full_ip[1].clone());
        }
        Microservice::new(
            id,
            name,
            state,
            ip,
            port
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

    fn get_ip_port(id: String) -> [String; 2] {
        let mut cmd = Command::new("docker");
        cmd.args(&["inspect", "-f", "'{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}'", &id]);
        let output = cmd.output().expect("failed to inspect container");
        let ip = String::from_utf8(output.stdout).expect("Inspecting ip did not work").trim().to_string();
        let ip = ip[1..ip.len()-1].to_string(); // 'ip' -> ip
        let mut cmd = Command::new("docker");
        cmd.args(&["port", &id]);
        let output = cmd.output().expect("failed to inspect container");
        let port = String::from_utf8(output.stdout).expect("Inspecting the port did not work").trim().to_string();
        [ip, port]
    }

    pub fn do_action(
        &self,
        action: MicroserviceAction
    ) -> Option<String> {
        let valid_action = match action {
            MicroserviceAction::Start => match self.state {
                MicroserviceState::Created => true,
                MicroserviceState::Exited => true,
                MicroserviceState::NotFound => {
                    if self.name != "eatup_server" {
                        false
                    }
                    else {
                        match cmd::run_server() {
                            Ok(_) => return None,
                            Err(e) => return Some(e)
                        }
                    }
                }
                _ => false
            },
            MicroserviceAction::Stop => match self.state {
                MicroserviceState::Running => true,
                MicroserviceState::Paused => true,
                MicroserviceState::Exited => true,
                _ => false
            },
            MicroserviceAction::Remove => match self.state {
                MicroserviceState::Running => true,
                MicroserviceState::Created => true,
                MicroserviceState::Exited => true,
                _ => false
            }
        };
        let action_string = action.to_string();
        if !valid_action {
            return Some(format!("Invalid action {} for {}", action_string, self.name));
        }
        if action == MicroserviceAction::Remove && self.state == MicroserviceState::Running {
            if let Some(e) = self.do_action(MicroserviceAction::Stop) {
                return Some(e);
            }
        }
        let mut cmd = Command::new("docker");
        cmd.args(&[&action_string, &self.name]);
        let output = cmd.output().expect("failed to execute docker command");
        let output = String::from_utf8(output.stdout).unwrap().trim().to_string();
        match self.name == output {
            true => None,
            false => Some(format!("Failed to {} {}", action_string, self.name))
        }
    }

    // getters
    pub fn get_state(&self) -> MicroserviceState {
        self.state.clone()
    }

    pub fn get_ip(&self) -> Option<String> {
        self.ip.clone()
    }
}
