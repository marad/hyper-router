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
    pub fn empty() -> Self {
        Router::new(vec![])
    }

    pub fn new(routes: Vec<Route>) -> Router {
        Router {
            routes,
            not_found_handler: handlers::default_404_handler,
            method_not_supported_handler: handlers::method_not_supported_handler,
        }
    }

    pub fn not_found(mut self, handler: Handler) -> Self {
        self.not_found_handler = handler;
        self
    }

    pub fn method_not_supported(mut self, handler: Handler) -> Self {
        self.method_not_supported_handler = handler;
        self
    }

    pub fn add(mut self, route: Route) -> Self {
        self.routes.push(route);
        self
    }

    pub fn find_handler(&self, request: &Request<Body>) -> (Handler, RouteParameters) {
        let mut path_match = None;
        let req_path = request.uri().path();
        let req_method = request.method();
        for route in &self.routes {
            let matches = route.path.matches(req_path);
            if matches.is_some() {
                if route.method == req_method {
                    return (route.handler, matches.unwrap());
                } else {
                    path_match = matches;
                }
            }
        }
        // At this point, no route matched both path AND method.
        // if path_match is true, we return method_not_supported, otherwise,
        // not found
        return if path_match.is_some() {
            (self.method_not_supported_handler, path_match.unwrap())
        } else {
            (self.not_found_handler, RouteParameters::none())
        };
    }
}

/// Builder for a router
///
/// Example usage:
///
#[derive(Debug)]
pub struct RouterBuilder {
    routes: Vec<Route>,
    not_found_handler: Handler,
    method_not_supported_handler: Handler,
}

impl RouterBuilder {
    pub fn new() -> Self {
        RouterBuilder {
            routes: vec![],
            not_found_handler: handlers::default_404_handler,
            method_not_supported_handler: handlers::method_not_supported_handler,
        }
    }

    /// Adds new `Route` for `Router` that is being built.
    ///
    /// Example:
    ///
    /// ```rust
    /// use hyper::{Request, Response, Body};
    /// use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH};
    /// use hyper_router::{Route, RouterBuilder};
    ///
    /// fn some_handler(_: Request<Body>) -> Response<Body> {
    ///     let body = "Hello World";
    ///     Response::builder()
    ///         .header(CONTENT_LENGTH, body.len() as u64)
    ///         .header(CONTENT_TYPE, "text/plain")
    ///         .body(Body::from(body))
    ///         .expect("Failed to construct the response")
    /// }
    ///
    /// RouterBuilder::new().add(Route::get("/hello").using(some_handler));
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, route: Route) -> Self {
        self.routes.push(route);
        self
    }

    pub fn not_found(mut self, handler: Handler) -> Self {
        self.not_found_handler = handler;
        self
    }

    pub fn method_not_supported(mut self, handler: Handler) -> Self {
        self.method_not_supported_handler = handler;
        self
    }

    pub fn build(self) -> Router {
        Router::new(self.routes)
            .method_not_supported(self.method_not_supported_handler)
            .not_found(self.not_found_handler)
    }
}
