use rocket::{Config, Build, Rocket, launch, routes, catchers, get};
use std::net::Ipv4Addr;
use rocket::serde::json::Json;

mod api;
mod model;
mod cmd;

pub const MICROSERVICES: [&'static str; 2] = [
    "eatup_db",
    "eatup_server"
];

pub const ENV: &'static str = "/installation/.env";

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
    let args: Vec<String> = std::env::args().collect();
    let port = match args.len() {
        2 => match args[1].parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                eprintln!("Invalid port number");
                std::process::exit(1);
            }
        }
        _ => 80
    };

    let config = Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        port,
        ..config()
    };
    rocket::custom(&config)
        .mount("/", routes![ping])
        .mount("/api/v1", api::get_v1_routes())
        .register("/api", catchers![api::error::not_implemented])
        .register("/", catchers![
            api::error::internal_server_error,
            api::error::not_found
        ])
}