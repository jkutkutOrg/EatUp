use warp::ws::{Message};
use warp::{Reply, Rejection, Error};
use futures::{FutureExt, StreamExt};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use serde_json::json;
use serde::{Serialize};

use crate::routes::{Router, Request, get_router};

pub type Socket = UnboundedSender::<Result<Message, Error>>;

pub async fn ws_handler(
    ws: warp::ws::Ws
) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(move |socket| async {
        println!("Socket connection\n");
        let router: Router = get_router();
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
                Ok(msg) => {
                    router.handle_request(
                        &client_sender,
                        Request::new(&msg.to_str().unwrap())
                    );
                },
                Err(e) => {
                    eprintln!("error receiving ws message: {}", e);
                    break;
                }
            }
        }
        println!("socket disconnected\n");
    }))
}

pub fn send_json(
    socket: &Socket,
    obj: &impl Serialize
) {
    socket.send(Ok(Message::text(
        json!(obj).to_string()
    ))).unwrap();
}

pub fn send_err(
    socket: &Socket,
    err: &str
) {
    socket.send(Ok(Message::text(
        json!({
            "error": err
        }).to_string()
    ))).unwrap();
}