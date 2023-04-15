use rocket::{Build, Rocket, launch, routes, catchers};
use std::env;
use rocket::tokio;
use tokio_postgres::NoTls;
use std::net::Ipv4Addr;

mod api;
mod db;
mod tools;
mod qr;

mod route_tools {
    use rocket::{get, catch, Responder};
    use rocket::serde::json::Json;

    #[get("/")]
    pub fn ping() -> Json<&'static str> {
        Json("Eatup up and running!")
    }

    #[catch(404)]
    pub fn qr_not_found() -> Json<&'static str> {
        Json("QR not found. Are you sure the QR should be valid?")
    }

    #[catch(501)]
    pub fn api_not_implemented() -> Json<&'static str> {
        Json("Ups, this feature is not implemented yet.")
    }

    #[catch(500)]
    pub fn internal_server_error() -> Json<&'static str> {
        Json("Well, this is embarrassing. Something went wrong on our side. Turns out, rust can fail too.")
    }

    #[derive(Responder)]
    #[response(status = 409, content_type = "json")]
    pub struct InvalidAPI {
        message: String
    }

    impl InvalidAPI {
        pub fn new(message: String) -> InvalidAPI {
            InvalidAPI { message }
        }
    }
}


#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv::from_path("/db/.env").ok();
    let db_properties = format!(
        "host={} port={} dbname={} user={} password={}",
        env::var("DB_IP").unwrap(),
        env::var("DB_PORT").unwrap(),
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

    let config = rocket::Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..rocket::Config::debug_default()
    };
    rocket::custom(&config)
        .manage(client)
        .mount("/", routes![route_tools::ping])
        .mount("/api/v1", api::get_v1_routes())
        .register("/api", catchers![route_tools::api_not_implemented])
        .mount("/", rocket::fs::FileServer::from("/db/public"))
        .register("/qr/", catchers![route_tools::qr_not_found, route_tools::internal_server_error])
}