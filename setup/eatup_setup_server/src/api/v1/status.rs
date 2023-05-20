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
    match cmd::create_db() {
        Ok(_) => Ok(Status::Ok),
        Err(e) => Err(InvalidAPI::new(e))
    }
}

#[post("/uninstall")]
pub async fn uninstall() -> Result<Status, Status> {
    Err(Status::NotImplemented) // TODO
}
