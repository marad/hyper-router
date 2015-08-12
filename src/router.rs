use hyper::method::Method;
use hyper::method::Method::{Get, Post};
use hyper::server::{Request, Response};
use hyper::uri::RequestUri;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::status::StatusCode;
use std::fmt;

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
        response.send(b"page not found").unwrap();
    }

    pub fn find_handler(&self, uri: &RequestUri) -> Handler {
        if let AbsolutePath(path) = uri.clone() {
            self.routes.iter().find(|route| {
                    path == route.path
                })
                .map(|route| route.handler)
                .unwrap_or(Router::default_404_handler)
        } else {
            // TODO: this should probably be different error (unsupported)
            Router::default_404_handler
        }
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
