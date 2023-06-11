use super::*;

#[get("/status")]
pub async fn get_status() -> Json<ProjectState>{
  Json(cmd::get_status())
}

#[post("/create")]
pub async fn create() -> Result<Status, InvalidAPI> {
  match cmd::project_create() {
    Ok(_) => Ok(Status::Ok),
    Err(e) => Err(InvalidAPI::new(e))
  }
}

mod model {
  use serde::Deserialize;

  #[derive(Debug, Deserialize)]
  pub struct InstallBody {
    pub db_usr: String,
    pub db_usr_passwd: String,
    pub server_port: u16,
  }
}

use model::InstallBody;

#[post("/install", format = "application/json", data = "<body>")]
pub async fn install(
  body: Json<InstallBody>
) -> Result<Status, InvalidAPI> {
  let body = body.into_inner();
  let status = cmd::get_status();
  match status {
    ProjectState::NotCreated => return Err(InvalidAPI::new(
      "Project not created".to_string()
    )),
    ProjectState::Installed => return Err(InvalidAPI::new(
      "Project already installed".to_string()
    )),
    _ => ()
  }
  cmd::create_env_file(
    body.db_usr,
    body.db_usr_passwd,
    body.server_port
  );
  if let Err(e) = cmd::create_db() {
    return Err(InvalidAPI::new(e));
  }
  // if let Err(e) = cmd::run_server() { // TODO
  //   return Err(InvalidAPI::new(e));
  // }
  Ok(Status::Ok)
}

#[post("/uninstall")]
pub async fn uninstall() -> Status {
  let status = cmd::get_status();
  if status == ProjectState::NotCreated {
    return Status::Ok;
  }
  if status == ProjectState::Installed {
    let s = cmd::microservice_action(MicroserviceAction::Remove, "eatup_db".to_string());
    if let Some(e) = s {
      println!("Failed to remove db container: {}", e.message());
      return Status::InternalServerError;
    }
    // remove env file: done later
  }
  match cmd::remove_dir_contents("/installation") {
    Ok(_) => Status::Ok,
    Err(e) => {
      println!("Failed to remove installation directory: {}", e);
      Status::InternalServerError
    }
  }
}
