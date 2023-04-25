use super::*;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Router {
    endpoint: Option<fn(&Socket, Request)>,
    routes: HashMap<String, Router>
}

impl Router {
    pub fn new(
        mut endpoints: Vec<(&str, fn (&Socket, Request))>
    ) -> Self {
        let mut endpoints = endpoints.into_iter()
            .map(|(endpoint, func)| {
                let req = Request::new(endpoint).unwrap();
                (req, func)
            })
            .collect::<Vec<(Request, fn (&Socket, Request))>>();
        Self::fill(endpoints, 0)
    }

    fn fill(
        mut endpoints: Vec<(Request, fn (&Socket, Request))>,
        depth: usize
    ) -> Self {
        let endpoint = Self::get_root_ft(&endpoints, depth);
        let mut routes = HashMap::new();
        endpoints = endpoints.into_iter()
            .filter(|(e, _)| e.endpoint.len() > depth)
            .collect::<Vec<(Request, fn (&Socket, Request))>>();
        while endpoints.len() > 0 {
            let q = endpoints[0].0.endpoint[depth].clone();
            let mut epts = Vec::new();
            let mut i = 0;
            while i < endpoints.len() {
                if endpoints[i].0.endpoint[depth] == q {
                    epts.push(endpoints.remove(i));
                } else {
                    i += 1;
                }
            }
            routes.insert(q, Self::fill(epts, depth + 1));
        }
        Self {endpoint,routes}
    }

    fn get_root_ft(
        endpoints: &Vec<(Request, fn (&Socket, Request))>,
        depth: usize
    ) -> Option<fn(&Socket, Request)> {
        let actual_endpoint = endpoints.iter()
            .filter(|(e, _)| e.endpoint.len() == depth)
            .collect::<Vec<&(Request, fn (&Socket, Request))>>();
        match actual_endpoint.len() {
            0 => None,
            1 => Some(actual_endpoint[0].1),
            _ => {
                let endpoints = actual_endpoint.iter()
                    .map(|(e, _)| e.endpoint.join("/"))
                    .collect::<Vec<String>>().join("\n");
                panic!("fill_recursive: multiple endpoints at depth: {}\n{}\n", depth, endpoints);
            }
        }
    }

    pub fn handle_request(
        &self,
        socket: &Socket,
        req: Result<Request, String>
    ) {
        let req = match req {
            Err(e) => {
                println!("Invalid request: {}", &e);
                socket.send(Ok(Message::text(e))).unwrap();
                return;
            },
            Ok(req) => req
        };
        // self.print(0, "/".to_string());
        self.handle_request_recursive(socket, req, 0);
    }

    fn handle_request_recursive(
        &self,
        socket: &Socket,
        req: Request,
        depth: usize
    ) {
        match depth.cmp(&req.endpoint.len()) {
            Ordering::Equal => {
                match &self.endpoint {
                    Some(endpoint) => endpoint(socket, req),
                    None => socket.send(Ok(Message::text("This endpoint does not exists"))).unwrap()
                }
            },
            Ordering::Greater => {
                socket.send(Ok(Message::text("This endpoint does not exists"))).unwrap();
                panic!("handle_request: depth is greater than endpoint length at endpoint: {}", req.endpoint.join("/"));
            },
            Ordering::Less => {
                match self.routes.get(&req.endpoint[depth]) {
                    Some(router) => router.handle_request_recursive(socket, req, depth + 1),
                    None => socket.send(Ok(Message::text("This endpoint does not exists"))).unwrap()
                }
            }
        }
    }
}
