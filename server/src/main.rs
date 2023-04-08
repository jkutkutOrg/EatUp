#[macro_use] extern crate rocket;
use std::net::Ipv4Addr;

#[get("/")]
fn hello() -> &'static str {
    "Eatup up and running!"
}

mod api {
    use rocket::serde::{Deserialize, Deserializer};

    #[derive(Debug, Deserialize, FromForm)]
    struct ProductQuery {
        #[field(name = "category")]
        categories: Vec<String>,
        #[field(name = "allergy")]
        allergies: Vec<String>,
    }

    #[get("/products?<filters..>")]
    fn products(filters: ProductQuery) -> String {
        let mut s: String = "v1 product".to_string();

        if filters.categories.len() > 1 {
            s.push_str(&format!(" with categories {:?}", filters.categories));
        } else if let Some(category) = filters.categories.get(0) {
            s.push_str(&format!(" with category {:?}", category));
        }
        else {
            s.push_str(" with no category");
        }

        if filters.allergies.len() > 1 {
            s.push_str(&format!(" with allergies {:?}", filters.allergies));
        } else if let Some(allergy) = filters.allergies.get(0) {
           s.push_str(&format!(" with allergy {:?}", allergy));
        }
        else {
            s.push_str(" with no allergy");
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
