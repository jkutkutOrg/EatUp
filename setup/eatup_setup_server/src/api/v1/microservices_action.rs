use super::*;
use super::MicroserviceAction;

#[post("/microservices/<action>/<name>")]
pub async fn microservice_action(
  action: MicroserviceAction,
  name: String,
) -> Result<Status, InvalidAPI> {
  match cmd::microservice_action(action, name) {
    Some(e) => Err(e),
    None => Ok(Status::Ok)
  }
}
