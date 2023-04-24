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
        endpoints.reverse();
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
        // TODO
        todo!("handle request");
    }

    fn fill_recursive(
        endpoints: Vec<(Request, fn (&Socket, Request))>,
        depth: usize
    ) -> Self {
        let mut router = Self {
            endpoint: None,
            routes: HashMap::new()
        };
        for (r, ft) in endpoints {
            match depth.cmp(&r.endpoint.len()) {
                Ordering::Equal => {
                    router.endpoint = Some(ft);
                },
                Ordering::Less => {
                    // TODO
                    todo!("Recursive fill");
                },
                Ordering::Greater => panic!("Depth is greater than endpoint length at endpoint: {}", r.endpoint.join("/"))
            }
        }
        router
    }
}
