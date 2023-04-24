use warp::ws::{Message};
use std::collections::HashMap;

use crate::{Socket};

mod router;
mod request;

pub use router::Router;
pub use request::Request;

pub fn get_router() -> Router {
    // let mut r = Router::new();
    // r.add_endpoint("/test", test_endpoint)
    //     .add_endpoint("/microservices", get_all_microservices);
    // r
    Router::new(
        vec![
            ("/".to_string(), ping), // TODO test
            ("/test/3".to_string(), test_endpoint2), // TODO test
            ("/test".to_string(), test_endpoint1),
            ("/test/2".to_string(), test_endpoint2),
            ("/microservices".to_string(), get_all_microservices)
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

fn test_endpoint1(
    socket: &Socket,
    _req: Request
) {
    let mut cmd = std::process::Command::new("ls");
    cmd.arg("-l");
    let output = cmd.output().unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    socket.send(Ok(Message::text(output))).unwrap();
}

fn test_endpoint2(
    socket: &Socket,
    _req: Request
) {
    let mut cmd = std::process::Command::new("ls");
    cmd.arg("-a");
    let output = cmd.output().unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    socket.send(Ok(Message::text(output))).unwrap();
}

fn get_all_microservices(
    socket: &Socket,
    _req: Request
) {
    let micros = crate::cmd::get_all_microservices();
    for micro in micros {
        socket.send(Ok(Message::text(format!("micro: {:?}", micro)))).unwrap();
    }
}