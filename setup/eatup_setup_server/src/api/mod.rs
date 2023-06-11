pub mod error;

mod v1;

pub fn get_v1_routes() -> Vec<rocket::Route> {
  v1::get_all_routes()
}