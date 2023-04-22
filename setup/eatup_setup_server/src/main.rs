use warp::ws::{Message};
use warp::{Reply, Filter, Rejection, Error};
use tokio_stream::wrappers::UnboundedReceiverStream;
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

use std::process::Command;

// use serde_json::json;

async fn incoming_msg(
    socket: &UnboundedSender::<Result<Message, Error>>,
    msg: Message
) {
    println!("msg: {:?}", msg);
    
    let mut cmd = Command::new("ls");
    cmd.arg("-al");

    let output = cmd.output().expect("failed to execute cmd");
    let msg = output.stdout;
    let msg = String::from_utf8(msg).unwrap();
    let msg = Message::text(msg);

    socket.send(Ok(msg)).unwrap();
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
