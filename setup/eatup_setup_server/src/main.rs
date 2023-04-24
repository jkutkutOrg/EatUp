use warp::ws::{Message};
use warp::{Reply, Filter, Rejection, Error};
use tokio_stream::wrappers::UnboundedReceiverStream;
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

type Socket = UnboundedSender::<Result<Message, Error>>;

mod cmd;

mod routes {
    use warp::ws::{Message};
    use warp::Future;
    use std::collections::HashMap;

    use crate::Socket;
    use crate::cmd;

    // type Route = &'static str;
    // pub struct Router<'r> {
    //     route: Route,
    //     endpoints: HashMap<Route, RouterEndpoint<'r>>
    // }
    // struct Endpoint<'e>
    // {
    //     func: &'e dyn Fn(&Socket, Request) -> dyn Future<Output = ()>
    // }
    // impl Endpoint<'_> {
    //     fn new(
    //         // ft: dyn Fn(&Socket, Request) -> dyn Future<Output = ()>
    //         ft: &dyn Fn(&Socket, Request) -> dyn Future<Output = ()>
    //     ) -> Self {
    //         Self {func: ft}
    //     }
    // }
    // enum RouterEndpoint<'re> {
    //     Endpoint(Endpoint<'re>),
    //     Router(Router<'re>)
    // }

    // impl Router<'_> {
    //     // fn new() -> Self {
    //     //     Self::new_switch("/")
    //     // }

    //     // fn new_switch(route: Route) -> Self {
    //     //     Self {
    //     //         route,
    //     //         endpoints: HashMap::new()
    //     //     }
    //     // }

    //     // TODO a way to add routes

    //     // pub fn add_endpoint<Fut>(
    //     //     &mut self,
    //     //     endpoint: &'static str,
    //     //     func: impl FnOnce(&Socket, Request) -> Fut
    //     // ) -> &mut Self 
    //     // where
    //     //     Fut: Future<Output = ()> + 'static
    //     // {
    //     //     let endpoint = endpoint.split("/").collect::<Vec<&str>>();
    //     //     println!("endpoint: {:?}", endpoint);
    //     //     self
    //     // }

    //     pub fn add_endpoint<Fut>(
    //         &mut self,
    //         endpoint: &'static str,
    //         func: Endpoint
    //     ) -> &mut Self
    //     {
    //         let endpoint = endpoint.split("/").collect::<Vec<&str>>();
    //         println!("endpoint: {:?}", endpoint);
    //         self
    //     }
    // }

    // pub fn get_router() -> Router<'static> {
    //     // Router::new()
    //     //     .add_endpoint("/microservices", test_endpoint)
    //     let mut r = Router {
    //         route: "/",
    //         endpoints: HashMap::new()
    //     };
    //     r.add_endpoint("/microservices", Endpoint::new(test_endpoint));
    //     r
    // }

    // ---------------------------------------------------

    // pub struct Method<C, F>
    // where
    //     C: Fn(&Socket, Request) -> F,
    //     F: std::future::Future,
    // {
    //     ft: C,
    // }

    // impl<C, F> Method<C, F>
    // where
    //     C: Fn(&Socket, Request) -> F,
    //     F: std::future::Future,
    // {
    //     pub fn new(
    //         ft: C,
    //     ) -> Self {
    //         Self { ft }
    //     }

    //     pub async fn exec(
    //         self,
    //         socket: &Socket,
    //         req: Request
    //     ) {
    //         (self.ft)(socket, req).await;
    //     }
    // }

    pub struct Method<C>
    where
        C: Fn(&Socket, Request)
    {
        ft: C,
    }

    impl<C> Method<C>
    where
        C: Fn(&Socket, Request)
    {
        pub fn new(
            ft: C,
        ) -> Self {
            Self { ft }
        }

        pub fn exec(
            self,
            socket: &Socket,
            req: Request
        ) {
            (self.ft)(socket, req);
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

    pub async fn get_router() -> String {
        "".to_string()
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

use routes::*; // TODO remove
async fn incoming_msg(
    socket: &Socket,
    msg: Message
) {
    let msg = msg.to_str().unwrap();
    println!("msg: {}", msg);

    let m = Method::new(test_endpoint);
    let req = Request::new(msg).unwrap();
    m.exec(socket, req);

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
    let mut r = routes::get_router().await;
    let routes = warp::path("ws")
        .and(warp::ws())
        .and_then(ws_handler)
        .with(warp::cors().allow_any_origin());
    println!("Running on port {port}");
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
