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

        Self::fill(endpoints, 0)
    }

    fn fill(
        endpoints: Vec<(Request, fn (&Socket, Request))>,
        depth: usize
    ) -> Self {
        // get childs
        // get routes from childs
        // if defined current, add endpoint
        // TODO
        Self {
            endpoint: Self::get_root_ft(&endpoints, depth), // TODO 
            routes: HashMap::new() // TODO
        }
    }

    fn get_root_ft(
        endpoints: &Vec<(Request, fn (&Socket, Request))>,
        depth: usize
    ) -> Option<fn(&Socket, Request)> {
        None // TODO
    }

    // fn fill(
    //     endpoints: Vec<(Request, fn (&Socket, Request))>,
    // ) -> Self {
    //     // 0 -> root -> "/"
    //     // 1 -> first level -> "/test"
    //     // 2 -> second level -> "/test/2"
    //     let actual_endpoint = endpoints.iter()
    //         .filter(|(e, _)| e.endpoint.len() == 0)
    //         .collect::<Vec<&(Request, fn (&Socket, Request))>>();
    //     let actual_endpoint = match actual_endpoint.len() {
    //         0 => None,
    //         1 => Some(actual_endpoint[0]),
    //         _ => {
    //             let endpoints = actual_endpoint.iter()
    //                 .map(|(e, _)| e.endpoint.join("/"))
    //                 .collect::<Vec<String>>();
    //             panic!("fill_recursive: multiple endpoints at depth: {}\n  {:?}", 0, endpoints);
    //         }
    //     };
    //     let mut router = Self {
    //         endpoint: actual_endpoint.map(|(_, ft)| *ft),
    //         routes: HashMap::new()
    //     };

    //     let mut endpoints = endpoints.into_iter()
    //         .filter(|(e, _)| e.endpoint.len() > 0)
    //         .collect::<Vec<(Request, fn (&Socket, Request))>>();

    //     // create next level
    //     while endpoints.len() > 0 {
    //         let (r, ft) = endpoints.remove(0);
    //         // check if endpoint exists
    //         match router.routes.get(&r.endpoint[0]) {
    //             Some(_) => (),
    //             None => {
    //                 router.routes.insert(
    //                     r.endpoint[0].clone(),
    //                     Self {
    //                         endpoint: None,
    //                         routes: HashMap::new()
    //                     }
    //                 );
    //             }
    //         };

    //         println!("Depth: {}", 0);
    //         println!("Endpoint: {:?} -> len: {}", &r.endpoint, &r.endpoint.len());

    //         let mut vec: Vec<(Request, fn (&Socket, Request))> = Vec::new();
    //         let mut i = 0;
    //         while i < endpoints.len() {
    //             if &endpoints[i].0.endpoint[0] == &r.endpoint[0] {
    //                 vec.push(endpoints.remove(i));
    //             } else {
    //                 i += 1;
    //             }
    //         }
    //         vec.push((r, ft));
    //         Self::fill_recursive(&router.routes.get_mut(&r.endpoint[0]).unwrap(), vec, 0 + 1);
    //     }
    //     router
    // }

    // fn fill_recursive(
    //     router: &mut Self,
    //     mut endpoints: Vec<(Request, fn (&Socket, Request))>,
    //     depth: usize
    // ) {
    //     // Check if endpoint exists
    //     let actual_endpoint = endpoints.iter()
    //         .filter(|(e, _)| e.endpoint.len() == depth)
    //         .collect::<Vec<&(Request, fn (&Socket, Request))>>();
    //     match actual_endpoint.len() {
    //         0 => (),
    //         1 => {
    //             router.endpoint = Some(actual_endpoint[0].1);
    //         },
    //         _ => {
    //             let endpoints = actual_endpoint.iter()
    //                 .map(|(e, _)| e.endpoint.join("/"))
    //                 .collect::<Vec<String>>();
    //             panic!("fill_recursive: multiple endpoints at depth: {}\n  {:?}", depth, endpoints);
    //         }
    //     };
    // }

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

    // fn fill_recursive(
    //     endpoints: Vec<(Request, fn (&Socket, Request))>,
    //     depth: usize
    // ) -> Self {
    //     let mut router = Self {
    //         endpoint: None,
    //         routes: HashMap::new()
    //     };
    //     let mut endpoints = endpoints.iter()
    //         .map(|(req, ft)| {
    //             (
    //                 Request {
    //                     endpoint: req.endpoint.clone(),
    //                     params: vec![]
    //                 },
    //                 *ft
    //             )
    //         })
    //         .collect::<Vec<(Request, fn (&Socket, Request))>>();
    //     while endpoints.len() > 0 {
    //         let (r, ft) = endpoints.remove(0);
    //         println!("Depth: {}", depth);
    //         println!("Endpoint: {:?} -> len: {}", &r.endpoint, &r.endpoint.len());
    //         match depth.cmp(&r.endpoint.len()) {
    //             Ordering::Equal => {
    //                 router.endpoint = Some(ft);
    //             },
    //             Ordering::Less => {
    //                 match router.routes.get_mut(&r.endpoint[depth]) {
    //                     Some(route) => {
    //                         let filtered_endpoints = endpoints.iter()
    //                             .map(|(req, ft)| {
    //                                 (
    //                                     Request {
    //                                         endpoint: req.endpoint.clone(),
    //                                         params: vec![]
    //                                     },
    //                                     ft.clone()
    //                                 )
    //                             })
    //                             .filter(|(req, _)| {
    //                                 for i in 0..r.endpoint.len() {
    //                                     if &i < &req.endpoint.len() && &req.endpoint[i] != &r.endpoint[i] {
    //                                         return false;
    //                                     }
    //                                 }
    //                                 true
    //                             })
    //                             .collect::<Vec<(Request, fn (&Socket, Request))>>();
    //                         endpoints = endpoints.iter()
    //                             .map(|(req, ft)| {
    //                                 (
    //                                     Request {
    //                                         endpoint: req.endpoint.clone(),
    //                                         params: vec![]
    //                                     },
    //                                     ft.clone()
    //                                 )
    //                             })
    //                             .filter(|(req, _)| {
    //                                 for i in 0..r.endpoint.len() {
    //                                     if &i < &req.endpoint.len() && &req.endpoint[i] != &r.endpoint[i] {
    //                                         return true;
    //                                     }
    //                                 }
    //                                 false
    //                             })
    //                             .collect::<Vec<(Request, fn (&Socket, Request))>>();
    //                         match route.routes.get_mut(&r.endpoint[depth + 1]) {
    //                             Some(rr) => {
    //                                 let new_router = Self::fill_recursive(filtered_endpoints, depth + 2);
    //                                 rr.routes.insert(r.endpoint[depth + 1].clone(), new_router);
    //                             },
    //                             None => {
    //                                 let new_router = Self::fill_recursive(filtered_endpoints, depth + 1);
    //                                 route.routes.insert(r.endpoint[depth + 1].clone(), new_router);
    //                             }
    //                         }
                            
    //                     },
    //                     None => {
    //                         let filtered_endpoints = endpoints.iter()
    //                             .map(|(req, ft)| {
    //                                 (
    //                                     Request {
    //                                         endpoint: req.endpoint.clone(),
    //                                         params: vec![]
    //                                     },
    //                                     ft.clone()
    //                                 )
    //                             })
    //                             .filter(|(req, _)| {
    //                                 for i in 0..r.endpoint.len() {
    //                                     if &i < &req.endpoint.len() && &req.endpoint[i] != &r.endpoint[i] {
    //                                         return false;
    //                                     }
    //                                 }
    //                                 true
    //                             })
    //                             .collect::<Vec<(Request, fn (&Socket, Request))>>();
    //                         endpoints = endpoints.iter()
    //                             .map(|(req, ft)| {
    //                                 (
    //                                     Request {
    //                                         endpoint: req.endpoint.clone(),
    //                                         params: vec![]
    //                                     },
    //                                     ft.clone()
    //                                 )
    //                             })
    //                             .filter(|(req, _)| {
    //                                 for i in 0..r.endpoint.len() {
    //                                     if &i < &req.endpoint.len() && &req.endpoint[i] != &r.endpoint[i] {
    //                                         return true;
    //                                     }
    //                                 }
    //                                 false
    //                             })
    //                             .collect::<Vec<(Request, fn (&Socket, Request))>>();
    //                         let new_router = Self::fill_recursive(filtered_endpoints, depth + 1);
    //                         router.routes.insert(r.endpoint[depth].clone(), new_router);
    //                         // remove filtered_endpoints from endpoints
    //                     }
    //                 }
    //             },
    //             Ordering::Greater => {
    //                 // println!("Depth: {}", depth);
    //                 // println!("Endpoint: {:?}", &r.endpoint);
    //                 // println!("Endpoint length: {}", &r.endpoint.len());
    //                 // router.print(0, String::from("/"));
    //                 panic!("Depth is greater than endpoint length at endpoint: {}", r.endpoint.join("/"))
    //             }
    //         }
    //     }
    //     router
    // }

    // -------------------------------------------

    fn print(&self, depth: usize, endpoint: String) {
        let offset = Self::offset(depth);
        println!("{}****", &offset);
        println!("{}*Router: {}", &offset, &endpoint);
        if let Some(_) = &self.endpoint {
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
