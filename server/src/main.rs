#[macro_use] extern crate rocket;
use std::net::Ipv4Addr;

#[get("/")]
fn hello() -> &'static str {
    "Eatup up and running!"
}

#[launch]
fn rocket() -> _ {
    let config = rocket::Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..rocket::Config::debug_default()
    };
    rocket::custom(&config)
        .mount("/", routes![hello])
        .mount("/public", rocket::fs::FileServer::from("/db/public"))
}
