#[macro_use] extern crate rocket;
use std::net::Ipv4Addr;

#[get("/")]
fn hello() -> &'static str {
    "Eatup up and running!"
}

mod api {
    use rocket::serde::{/*Serialize,*/ Deserialize};

    #[derive(Debug, Deserialize, FromForm)]
    #[form(field = "categories", field = "allergies")]
    struct ProductQuery {
        categories: Option<String>,
        allergies: Option<String>,
    }

    #[get("/products?<filters..>")]
    fn products(filters: ProductQuery) -> String {
        let mut s: String = "v1 product".to_string();

        match filters.categories {
            Some(c) => s.push_str(&format!(" with categories {:?}", c)),
            None => (),
        }
        match filters.allergies {
            Some(a) => s.push_str(&format!(" with allergies {:?}", a)),
            None => (),
        }
        s
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
            // products_with_categories,
            // products_with_allergies,
            // products_without_filters,
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
