use super::*;

#[derive(Serialize)]
pub struct Microservice {
    id: Option<String>,
    name: String,
    state: MicroserviceState,
}

impl Microservice {
    pub fn new(
        id: Option<String>,
        name: String,
        state: MicroserviceState,
    ) -> Self {
        Microservice {
            id,
            name,
            state,
        }
    }

    pub fn by_name(name: String) -> Self {
        let id = Microservice::get_microservice_id(&name);
        Microservice::new(
            id.clone(),
            name.clone(),
            match id {
                Some(id) => MicroserviceState::new_by_id(&id),
                None => MicroserviceState::NotFound
            },
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

    pub fn do_action(
        &self,
        action: MicroserviceAction
    ) -> Option<String> {
        let valid_action = match action {
            MicroserviceAction::Start => match self.state {
                MicroserviceState::Created => true,
                MicroserviceState::Exited => true,
                _ => false
            },
            MicroserviceAction::Stop => match self.state {
                MicroserviceState::Running => true,
                MicroserviceState::Paused => true,
                MicroserviceState::Exited => true,
                _ => false
            }
        };
        let action_string = action.to_string();
        if !valid_action {
            return Some(format!("Invalid action {} for {}", action_string, self.name));
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
}
