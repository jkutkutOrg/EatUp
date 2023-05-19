use rocket::{Config, Build, Rocket, launch, routes, catchers, get};
// use std::env; // TODO
use std::net::Ipv4Addr;
use rocket::serde::json::Json;

mod api;
mod model;
mod cmd;

#[get("/")]
fn ping() -> Json<&'static str> {
    Json("eatup_setup_server up and running!")
}

#[cfg(debug_assertions)]
fn config() -> Config {
    Config::debug_default()
}

#[cfg(not(debug_assertions))]
fn config() -> rocket::Config {
    Config::release_default()
}

#[launch]
async fn rocket() -> Rocket<Build> {
    // dotenv::from_path(".env").ok();
    // env::var("DB_IP").unwrap();
    
    let config = Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..config()
    };
    rocket::custom(&config)
        //.manage() // TODO
        .mount("/", routes![ping])
        .mount("/api/v1", api::get_v1_routes())
        .register("/api", catchers![api::error::not_implemented])
        //.mount("/", rocket::fs::FileServer::from("/public")) // TODO
        .register("/", catchers![
            api::error::internal_server_error,
            api::error::not_found
        ])
}