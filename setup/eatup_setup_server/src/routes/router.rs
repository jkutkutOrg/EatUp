use super::*;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Router {
    endpoint: Option<fn(&Socket, Request)>,
    routes: HashMap<String, Router>
}

impl Router {
    pub fn new(
        mut endpoints: Vec<(String, fn (&Socket, Request))>
    ) -> Self {
        endpoints.sort_by(|(a, _), (b, _)| a.cmp(b));
        // endpoints.reverse();
        let endpoints = endpoints.into_iter()
            .map(|(endpoint, func)| {
                let req = Request::new(&endpoint).unwrap();
                (req, func)
            })
            .collect::<Vec<(Request, fn (&Socket, Request))>>();
        
        for (e, _) in &endpoints { // TODO debug
            println!("{:?}", &e);
        }

        Self::fill_recursive(endpoints, 0)
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
        println!("Handling request: {:?}", &req);
        self.print(0, "/".to_string());
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

    // -------------------------------------------

    fn print(&self, depth: usize, endpoint: String) {
        let offset = Self::offset(depth);
        println!("{}****", &offset);
        println!("{}*Router: {}", &offset, &endpoint);
        if let Some(end) = &self.endpoint {
            println!("{}*- endpoint: Some", &offset);
        }
        else {
            println!("{}*- endpoint: None", &offset);
        }
        for (route, router) in &self.routes {
            println!("{}*- routes: {}", &offset, route);
            router.print(depth + 1, format!("{}->{}", &endpoint, &route));
        }
        if self.routes.len() == 0 {
            println!("{}*- routes: None", &offset);
        }
        println!("{}****", &offset);
    }

    fn offset(depth: usize) -> String {
        let mut offset = String::new();
        for _ in 0..depth {
            offset.push_str("  ");
        }
        offset
    }
}
