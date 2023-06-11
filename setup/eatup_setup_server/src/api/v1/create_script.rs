use super::*;
use rocket::fs::TempFile;

#[patch("/create/script", format = "application/x-sql", data = "<_sql_script>")]
pub async fn create_script(
  mut _sql_script: TempFile<'_>
) -> Result<Status, InvalidAPI> {
  // let status = cmd::get_status();
  // if status != ProjectState::Created {
  //   return Err(InvalidAPI::new(
  //     format!("Project is not in the {:?} state", ProjectState::Created)
  //   ));
  // }
  // match sql_script.persist_to("/installation/load_db.sql").await {
  //   Ok(_) => Ok(Status::Ok),
  //   Err(e) => Err(InvalidAPI::new(
  //     format!("Failed to override sql script: {}", e)
  //   ))
  // }
  Ok(Status::NotImplemented) // TODO
}