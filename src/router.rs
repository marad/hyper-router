extern crate regex;

use hyper::method::Method;
use hyper::method::Method::{Get, Post};
use hyper::server::{Request, Response};
use hyper::uri::RequestUri;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::status::StatusCode;
use std::fmt;
use self::regex::Regex;

pub type Handler = fn(Request, Response);

pub struct Route {
    pub method: Method,
    pub path: String,
    pub handler: Handler
}

impl fmt::Debug for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Route {{method: {:?}, path: {:?}}}", self.method, self.path)
    }
}

#[derive(Debug)]
pub struct Router {
    routes: Vec<Route>,
}


impl Router {

    pub fn default_404_handler(request: Request, mut response: Response) {
        {*response.status_mut() = StatusCode::NotFound}
        response.send(b"page not found").ok();
    }

    pub fn method_not_supported_handler(request: Request, mut response: Response) {
        {*response.status_mut() = StatusCode::MethodNotAllowed}
        response.send(b"method not supported").ok();
    }

    pub fn internal_server_error_handler(_: Request, mut response: Response) {
        {*response.status_mut() = StatusCode::InternalServerError}
        response.send(b"internal server error").ok();
    }

    pub fn not_implemented_handler(_: Request, mut response: Response) {
        {*response.status_mut() = StatusCode::NotImplemented}
        response.send(b"not implemented").ok();
    }

    pub fn find_handler(&self, request: &Request) -> Handler {
        if let AbsolutePath(request_path) = request.uri.clone() {
            let matching_routes = self.find_matching_routes(&request_path);
            match matching_routes.len() {
                x if x <= 0 => Router::default_404_handler,
                1 => {
                    let method = request.method.clone();
                    matching_routes.iter()
                        .find(|route| route.method == method)
                        .map(|route| route.handler)
                        .unwrap_or(Router::method_not_supported_handler)
                }
                _ => Router::internal_server_error_handler
            }
        } else {
            Router::not_implemented_handler
        }
    }

    fn find_matching_routes(&self, request_path: &str) -> Vec<&Route> {
        self.routes.iter()
            .filter(|route| {
                // TODO: matcher should probably be in Route struct?
                let regex = Regex::new(&route.path).unwrap();
                regex.is_match(&request_path)
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct RouterBuilder {
    routes: Vec<Route>,
}

impl RouterBuilder {
    pub fn new() -> RouterBuilder {
        RouterBuilder { routes: vec![] }
    }

    pub fn add(mut self, route: Route) -> RouterBuilder {
        self.routes.push(route);
        self
    }

    pub fn build(self) -> Router {
        Router { routes: self.routes }
    }
}


#[cfg(test)]
mod tests {
    use hyper::method::Method::{Get,Post};
    use hyper::server::{Request, Response};
    use super::{Route, RouterBuilder};

    fn test_handler(req: Request, mut res: Response) {
        res.send(b"Hello World").unwrap();
    }

    #[test]
    fn test_api() {
        let router = RouterBuilder::new()
            .add(Route {
                method: Get,
                path: "/hello".to_string(),
                handler: test_handler
            })
            .add(Route {
                method: Post,
                path: "/test".to_string(),
                handler: test_handler
            })
            .build();

        println!("{:?}", router);
    }
}
