use std::collections::HashMap;

use crate::ws::{Socket, send_json, send_err};
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
    send_json(socket, &"pong");
}

fn get_all_microservices(
    socket: &Socket,
    _req: Request
) {
    let micros = crate::cmd::get_all_microservices();
    for micro in micros {
        send_json(socket, &micro);
    }
}