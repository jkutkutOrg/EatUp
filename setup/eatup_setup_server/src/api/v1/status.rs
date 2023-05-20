use super::*;

#[get("/status")]
pub async fn get_status() -> Json<ProjectState>{
    Json(cmd::get_status())
}

#[post("/create")]
pub async fn create() -> Result<Status, Status> {
    // TODO Create public directory
    // TODO Copy start script
    Err(Status::NotImplemented) // TODO
}

#[post("/install")]
pub async fn install() -> Result<Status, Status> {
    // TODO Create docker container
    Err(Status::NotImplemented) // TODO
}

#[post("/uninstall")]
pub async fn uninstall() -> Result<Status, Status> {
    Err(Status::NotImplemented) // TODO
}
