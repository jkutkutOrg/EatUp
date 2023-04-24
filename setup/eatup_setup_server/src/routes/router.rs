use super::*;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Router {
    endpoints: HashMap<String, RouterEndpoint>
}

#[derive(Clone)]
enum RouterEndpoint {
    Endpoint(
        fn (&Socket, Request)
    ),
    Router(Router)
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
        self.recursive_add_endpoint(
            Request::new(endpoint).unwrap(),
            func,
            0
        );
        self
    }

    pub fn handle_request(
        &self,
        socket: &Socket,
        req: Result<Request, String>
    ) {
        let req = match req {
            Ok(req) => req,
            Err(e) => {
                socket.send(Ok(Message::text(e))).unwrap();
                return;
            }
        };
        self.recursive_handle_request(socket, req, 0);
    }

    fn recursive_add_endpoint(
        &mut self,
        req: Request,
        func: fn (&Socket, Request),
        depth: usize
    ) {
        match depth.cmp(&(req.endpoint.len() - 1)) {
            Ordering::Equal => { // Overwrite endpoint
                self.endpoints.insert(
                    req.endpoint[depth].clone(),
                    RouterEndpoint::Endpoint(func)
                );
            },
            Ordering::Less => {
                let current = req.endpoint[depth].clone();
                match self.endpoints.get(&current) {
                    None => {
                        let mut router = Router::new();
                        router.recursive_add_endpoint(req, func, depth + 1);
                        self.endpoints.insert(current, RouterEndpoint::Router(router));
                    },
                    Some(endpoint) => {
                        match endpoint {
                            RouterEndpoint::Endpoint(_) => {
                                let mut router = Router::new();
                                router.recursive_add_endpoint(req, func, depth + 1);
                                self.endpoints.insert(current, RouterEndpoint::Router(router));
                            },
                            RouterEndpoint::Router(r) => {
                                r.recursive_add_endpoint(req, func, depth + 1);
                            }
                        }
                    }
                }
            },
            Ordering::Greater => panic!("Ups, I don't know what to do")
        }
    }

    fn recursive_handle_request(
        &self,
        socket: &Socket,
        req: Request,
        depth: usize
    ) {
        if depth >= req.endpoint.len() {
            socket.send(Ok(Message::text("Endpoint not found"))).unwrap();
            return;
        }
        match self.endpoints.get(&req.endpoint[depth]) {
            None => socket.send(Ok(Message::text("Endpoint not found"))).unwrap(),
            Some(endpoint) => {
                match endpoint {
                    RouterEndpoint::Endpoint(ft) => ft(socket, req),
                    RouterEndpoint::Router(r) => r.handle_request(socket, Ok(req))
                }
            }
        }
    }
}