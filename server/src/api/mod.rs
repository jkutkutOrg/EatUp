mod v1;

pub use v1::ProductQuery;
pub use v1::SessionQuery;

pub fn get_v1_routes() -> Vec<rocket::Route> {
    v1::get_all_routes()
}