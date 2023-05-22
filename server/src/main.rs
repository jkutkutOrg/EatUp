use rocket::{Config, Build, Rocket, launch, routes, catchers, get};
use std::env;
use rocket::tokio;
use tokio_postgres::NoTls;
use std::net::Ipv4Addr;
use rocket::serde::json::Json;

mod api;
mod db;
mod tools;
mod qr;

use tools::route_error;

pub const ENV: &'static str = "/installation/.env";
pub const PUBLIC_DIR: &'static str = "/installation/public";

#[get("/")]
fn ping() -> Json<&'static str> {
    Json("Eatup up and running!")
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
    let args = env::args().collect::<Vec<String>>();
    match args.len() {
        4 => (),
        _ => {
            eprintln!("Usage: {} <port> <db_host> <db_port>", args[0]);
            std::process::exit(1);
        }
    }

    let port = match args[1].parse::<u16>() {
        Ok(port) => port,
        Err(_) => {
            eprintln!("Invalid port number");
            std::process::exit(1);
        }
    };
    let db_host = &args[2];
    let db_port = match args[3].parse::<u16>() {
        Ok(port) => port,
        Err(_) => {
            eprintln!("Invalid db_port number");
            std::process::exit(1);
        }
    };

    dotenv::from_path(ENV).ok();

    let db_properties = format!(
        "host={} port={} dbname={} user={} password={}",
        db_host, db_port,
        env::var("DB_NAME").unwrap(),
        env::var("DB_USER").unwrap(),
        env::var("DB_USER_PASSWD").unwrap(),
    );

    let (client, connection) = match tokio_postgres::connect(&db_properties, NoTls).await {
        Ok((client, connection)) => (client, connection),
        Err(e) => {
            eprintln!("Not able to connect with DB:\n{}", e);
            eprintln!("Is the DB running?");
            std::process::exit(1);
        }
    };
 
    // Create the thread that will handle the communication with the database
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
            eprintln!("Stopping the server...");
            std::process::exit(1);
        }
    });

    let config = Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        port,
        ..config()
    };
    rocket::custom(&config)
        .manage(client)
        .mount("/", routes![ping])
        .mount("/api/v1", api::get_v1_routes())
        .register("/api", catchers![route_error::api_not_implemented])
        .mount("/", rocket::fs::FileServer::from(PUBLIC_DIR))
        .register("/qr/", catchers![route_error::qr_not_found, route_error::internal_server_error])
}