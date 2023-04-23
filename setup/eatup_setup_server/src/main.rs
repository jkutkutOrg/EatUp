use warp::ws::{Message};
use warp::{Reply, Filter, Rejection, Error};
use tokio_stream::wrappers::UnboundedReceiverStream;
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

use std::process::Command;

mod cmd;

async fn get_all_microservices(
    socket: &UnboundedSender::<Result<Message, Error>>
) {
    let micros = cmd::get_all_microservices();
    for micro in micros {
        println!("micro: {:?}", micro);
        socket.send(Ok(Message::text(format!("micro: {:?}", micro)))).unwrap();
    }
}

async fn incoming_msg(
    socket: &UnboundedSender::<Result<Message, Error>>,
    msg: Message
) {
    let msg = msg.to_str().unwrap();
    println!("msg: {}", msg);

    match msg.as_str() {
        "/microservices" => get_all_microservices(socket).await,
        _ => {
            println!("Unknown command");
            socket.send(Ok(Message::text("Unknown command"))).unwrap();
        }
    }
}

async fn ws_handler(ws: warp::ws::Ws) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(move |socket| async {
        println!("Socket connection\n");
        let (client_ws_sender, mut client_ws_rcv) = socket.split();
        let (client_sender, client_rcv) = unbounded_channel();
        let client_rcv = UnboundedReceiverStream::new(client_rcv);
        tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
            if let Err(e) = result {
                eprintln!("error sending websocket msg: {}", e);
            }
        }));
        while let Some(result) = client_ws_rcv.next().await {
            match result {
                Ok(msg) => incoming_msg(&client_sender, msg).await,
                Err(e) => {
                    eprintln!("error receiving ws message: {}", e);
                    break;
                }
            }
        }
        println!("socket disconnected\n");
    }))
}

#[tokio::main]
async fn main() {
    let port = 9000;
    let routes = warp::path("ws")
        .and(warp::ws())
        .and_then(ws_handler)
        .with(warp::cors().allow_any_origin());
    println!("Running on port {port}");
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
