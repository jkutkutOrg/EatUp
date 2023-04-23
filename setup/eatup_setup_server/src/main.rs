use warp::ws::{Message};
use warp::{Reply, Filter, Rejection, Error};
use tokio_stream::wrappers::UnboundedReceiverStream;
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

type Socket = UnboundedSender::<Result<Message, Error>>;

mod cmd;

mod routes {
    use warp::{Error};
    use warp::ws::{Message};
    use tokio::sync::mpsc::{UnboundedSender};

    use crate::Socket;
    use crate::cmd;

    enum Router {
        Route({
            route: &'static str,
            endpoints: HashMap<&'static str, Router>
        }),
        Endpoint(func: fn(&Socket, Request))
    }

    impl Router {
        // TODO main route

        fn new_route(route: &'static str) -> Self {
            Self::Route({
                route,
                endpoints: HashMap::new()
            })
        }

        fn new_endpoint(func: fn(&Socket, Request)) -> Self {
            Self::Endpoint(func)
        }

        // TODO a way to add routes
    }

    struct Request {
        endpoint: Vec<&str>,
        params: Vec<&str>
    }

    impl Request {
        fn new(socket_msg: &str) -> Result<Self, String> {
            let split = socket_msg.split("?").collect::<Vec<&str>>();
            let (endpoint, params) = match split.len() {
                1 => (split[0], vec![]),
                2 => (split[0], split[1].split("&").collect::<Vec<&str>>()),
                _ => return Err("Invalid request".to_string())
            }
            let endpoint = endpoint.split("/").collect::<Vec<&str>>();
            // TODO validation
            Ok(Self {
                endpoint,
                params
            })
        }
    }

    // ----------------- Routes -----------------

    pub async fn get_all_microservices(
        socket: &Socket
    ) {
        let micros = cmd::get_all_microservices();
        for micro in micros {
            println!("micro: {:?}", micro);
            socket.send(Ok(Message::text(format!("micro: {:?}", micro)))).unwrap();
        }
    }
}

async fn incoming_msg(
    socket: &Socket,
    msg: Message
) {
    let msg = msg.to_str().unwrap();
    println!("msg: {}", msg);

    match msg {
        "/microservices" => routes::get_all_microservices(socket).await,
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
                Ok(msg) => {
                    incoming_msg(&client_sender, msg).await; // TODO
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
