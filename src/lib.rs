extern crate hyper;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::server::{Request, Response};

mod path;
mod route;
mod builder;
pub mod handlers;

pub use self::path::Path;
pub use self::route::Route;
pub use self::builder::RouterBuilder;

pub type Handler = fn(Request, Response);

#[derive(Debug)]
pub struct Router {
    routes: Vec<Route>
}

impl Router {
    pub fn find_handler(&self, request: &Request) -> Handler {
        if let AbsolutePath(request_path) = request.uri.clone() {
            let matching_routes = self.find_matching_routes(&request_path);
            match matching_routes.len() {
                x if x <= 0 => handlers::default_404_handler,
                1 => {
                    let method = request.method.clone();
                    matching_routes.iter()
                        .find(|route| route.method == method)
                        .map(|route| route.handler)
                        .unwrap_or(handlers::method_not_supported_handler)
                }
                _ => handlers::internal_server_error_handler
            }
        } else {
            handlers::not_implemented_handler
        }
    }

    pub fn find_matching_routes(&self, request_path: &str) -> Vec<&Route> {
        self.routes.iter()
            .filter(|route| {
                route.path.matcher.is_match(&request_path)
            })
            .collect()
    }
}
