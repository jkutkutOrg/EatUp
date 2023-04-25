use warp::ws::{Message};
use std::collections::HashMap;
use serde_json::json;

use crate::{Socket};

mod router;
mod request;

pub use router::Router;
pub use request::Request;

pub fn get_router() -> Router {
    Router::new(
        vec![
            ("/", ping),
            ("/microservices", get_all_microservices)
        ]
    )
}

// ----------------- Routes -----------------

fn ping(
    socket: &Socket,
    _req: Request
) {
    socket.send(Ok(Message::text("pong"))).unwrap();
}

fn get_all_microservices(
    socket: &Socket,
    _req: Request
) {
    let micros = crate::cmd::get_all_microservices();
    for micro in micros {
        socket.send(Ok(Message::text(
            json!(micro).to_string()
        ))).unwrap();
    }
}