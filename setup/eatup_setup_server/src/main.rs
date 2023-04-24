use warp::ws::{Message};
use warp::{Reply, Filter, Rejection, Error};
use tokio_stream::wrappers::UnboundedReceiverStream;
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

type Socket = UnboundedSender::<Result<Message, Error>>;

mod cmd;
use routes::{Router, Request};

mod routes {
    use warp::ws::{Message};
    use warp::Future;
    use std::collections::HashMap;

    use crate::Socket;
    use crate::cmd;

    #[derive(Clone)]
    enum RouterEndpoint {
        Endpoint(
            fn (&Socket, Request)
        ),
        Router(Router)
    }

    #[derive(Clone)]
    pub struct Router {
        endpoints: HashMap<String, RouterEndpoint>
    }

    impl Router {
        pub fn new() -> Self {
            Self {
                endpoints: HashMap::new()
            }
        }

        pub fn add_endpoint(
            &mut self,
            endpoint: &str,
            func: fn (&Socket, Request)
        ) -> &mut Self {
            self.endpoints.insert(
                endpoint.to_string(),
                RouterEndpoint::Endpoint(func)
            );
            // TODO route to subrouter
            self
        }

        pub fn handle_request(
            &self,
            socket: &Socket,
            req: Request
        ) {
            match self.endpoints.get("/test").unwrap() {
                RouterEndpoint::Endpoint(ft) => ft(socket, req), // TODO
                RouterEndpoint::Router(r) => panic!("Router not implemented")
            }
        }
    }

    pub struct Request {
        endpoint: Vec<String>,
        params: Vec<String>
    }

    impl Request {
        pub fn new(socket_msg: &str) -> Result<Self, String> {
            let split = socket_msg.split("?").collect::<Vec<&str>>();
            let (endpoint, params) = match split.len() {
                1 => (split[0], vec![]),
                2 => (split[0], split[1].split("&").collect::<Vec<&str>>()),
                _ => return Err("Invalid request".to_string())
            };
            let endpoint = endpoint.split("/").collect::<Vec<&str>>();
            // TODO validation
            let endpoint = endpoint.iter().map(|s| s.to_string()).collect::<Vec<String>>();
            let params = params.iter().map(|s| s.to_string()).collect::<Vec<String>>();
            Ok(Self {endpoint, params})
        }
    }

    pub fn get_router() -> Router {
        let mut r = Router::new();
        r.add_endpoint("/test", test_endpoint)
            .add_endpoint("/microservices", get_all_microservices);
        r
    }

    // ----------------- Routes -----------------

    pub fn test_endpoint(
        socket: &Socket,
        req: Request
    ) {
        let mut cmd = std::process::Command::new("ls");
        cmd.arg("-l");
        let output = cmd.output().unwrap();
        let output = String::from_utf8(output.stdout).unwrap();
        socket.send(Ok(Message::text(output))).unwrap();
    }

    pub fn get_all_microservices(
        socket: &Socket,
        req: Request
    ) {
        let micros = cmd::get_all_microservices();
        for micro in micros {
            println!("micro: {:?}", micro);
            socket.send(Ok(Message::text(format!("micro: {:?}", micro)))).unwrap();
        }
    }
}

async fn ws_handler(
    ws: warp::ws::Ws
) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(move |socket| async {
        println!("Socket connection\n");
        let router: Router = routes::get_router();
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
                        Request::new(&msg.to_str().unwrap()).unwrap()
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
