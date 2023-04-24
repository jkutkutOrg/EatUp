use super::*;

pub struct Request {
    endpoint: Vec<String>,
    params: Vec<String>
}

impl Request {
    fn process_parameter(
        separator: &str,
        param: &str
    ) -> Vec<String> {
        param.split(separator)
            .filter(|s| s.len() > 0)
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    pub fn new(socket_msg: &str) -> Result<Self, String> {
        let split = socket_msg.split("?").collect::<Vec<&str>>();
        let (endpoint, params) = match split.len() {
            1 => (split[0], ""),
            2 => (split[0], split[1]),
            _ => return Err("Invalid request".to_string())
        };
        let endpoint = Self::process_parameter("/", endpoint);
        let params = Self::process_parameter("&", params);
        // TODO validation
        println!("New request:");
        println!("  - endpoint: {:?}", endpoint);
        println!("  - params: {:?}", params);
        Ok(Self {endpoint, params})
    }
}