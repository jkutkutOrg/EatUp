use super::*;

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