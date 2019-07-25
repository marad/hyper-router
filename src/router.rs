use crate::handlers;
use crate::parameters::RouteParameters;
use crate::route::Route;
use crate::Handler;
use hyper::Body;
use hyper::Request;

#[derive(Debug)]
pub struct Router {
    routes: Vec<Route>,
    not_found_handler: Handler,
    method_not_supported_handler: Handler,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: vec![],
            not_found_handler: handlers::default_404_handler,
            method_not_supported_handler: handlers::method_not_supported_handler,
        }
    }

    pub fn not_found(mut self, handler: Handler) -> Router {
        self.not_found_handler = handler;
        self
    }

    pub fn method_not_supported(mut self, handler: Handler) -> Router {
        self.method_not_supported_handler = handler;
        self
    }

    pub fn add(mut self, route: Route) -> Router {
        self.routes.push(route);
        self
    }

    pub fn find_handler(&self, request: &Request<Body>) -> (Handler, RouteParameters) {
        let req_path = request.uri().path();
        let req_method = request.method();
        for route in &self.routes {
            let matches = route.path.matches(req_path);
            if matches.is_some() {
                if route.method == req_method {
                    return (route.handler, matches.unwrap());
                } else {
                    return (self.method_not_supported_handler, matches.unwrap());
                }
            }
        }
        (self.not_found_handler, RouteParameters::new(vec![]))
    }
}
