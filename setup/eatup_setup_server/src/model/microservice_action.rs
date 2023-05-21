use rocket::request::FromParam;

use crate::api::error::InvalidAPI;

#[derive(Debug, PartialEq)]
pub enum MicroserviceAction {
    Start,
    Stop,
    Remove,
}

impl FromParam<'_> for MicroserviceAction {
    type Error = InvalidAPI;

    fn from_param(param: &'_ str) -> Result<Self, Self::Error> {
        match param {
            "start" => Ok(MicroserviceAction::Start),
            "stop" => Ok(MicroserviceAction::Stop),
            _ => Err(InvalidAPI::new("Invalid microservice action".to_string()))
        }
    }
}

impl MicroserviceAction {
    pub fn to_string(&self) -> String {
        match self {
            MicroserviceAction::Start => "start".to_string(),
            MicroserviceAction::Stop => "stop".to_string(),
            MicroserviceAction::Remove => "rm".to_string(),
        }
    }
}