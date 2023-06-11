use super::*;
use crate::model::Microservice;

#[get("/microservices")]
pub fn get_all_microservices(
) -> Result<Json<Vec<Microservice>>, Status> {
  Ok(Json(cmd::get_all_microservices()))
}