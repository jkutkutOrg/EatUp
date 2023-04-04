#[macro_use] extern crate rocket;
use std::net::Ipv4Addr;

#[get("/")]
fn hello() -> &'static str {
    "Eatup up and running!"
}

mod api {
    // Products
    #[get("/products")]
    fn products() -> &'static str {
        "v1 product"
    }

    // Sessions
    #[get("/sessions")]
    fn sessions() -> &'static str {
        "v1 sessions"
    }

    #[post("/sessions/<table_id>")]
    fn create_session(table_id: u32) -> String {
        format!("v1 create session for table {}", table_id)
    }

    #[patch("/sessions/<session_id>/end")]
    fn end_session(session_id: u32) -> String {
        format!("v1 end session {}", session_id)
    }

    // Orders
    #[get("/orders/<session_id>")]
    fn orders(session_id: u32) -> String {
        format!("v1 orders for session {}", session_id)
    }

    #[post("/orders/<session_id>")]
    fn create_order(session_id: u32) -> String {
        format!("v1 create order for session {}", session_id)
    }

    pub fn get_all_routes() -> Vec<rocket::Route> {
        routes![
            products,
            sessions,
            create_session,
            end_session,
            orders,
            create_order
        ]
    }

}

#[launch]
fn rocket() -> _ {
    let config = rocket::Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..rocket::Config::debug_default()
    };
    rocket::custom(&config)
        .mount("/", routes![hello])
        .mount("/api/v1", api::get_all_routes())
        .mount("/public", rocket::fs::FileServer::from("/db/public"))
}
