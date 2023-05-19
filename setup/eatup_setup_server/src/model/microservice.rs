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

    pub fn new_by_name(name: String) -> Self {
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
}
