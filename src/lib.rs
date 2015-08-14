#![doc(html_root_url = "https://marad.github.io/hyper-router/doc/hyper_router")]

//! # Hyper Router
//!
//! This cargo is a small extension to the great Hyper HTTP library. It basically is
//! adds the ability to define routes to request handlers and then query for the handlers
//! by request path.
//!
//! ## Usage
//!
//! To use the library just add: 
//! 
//! ```
//! hyper-router = "*"
//! ```
//!
//! to your dependencies.
//!
//! ```rust
//! extern crate hyper;
//! extern crate hyper_router;
//!
//! use hyper::server::{Server, Request, Response};
//! use hyper_router::{Route, RouterBuilder};
//!
//! fn basic_handler(_: Request, res: Response) {
//!   res.send(b"Hello World!").unwrap();
//! }
//!
//! fn main() {
//!   let router = RouterBuilder::new()
//!     .add(Route::get("/greet").using(basic_handler))
//!     .build();
//!
//!   Server::http("0.0.0.0:8080").unwrap()
//!     .handle(move |request: Request, response: Response| {
//!       let handler = router.find_handler(&request);
//!       handler(request, response);
//!     }).unwrap();
//! }
//! ```
//!
//! This code will start Hyper server and add use router to find handlers for request.
//! We create the `Route` so that when we visit path `/greet` the `basic_handler` handler
//! will be called.
//!
//! ## Things to note
//!
//! * `Path::new` method accepts regular expressions so you can match every path you please.
//! * If you have request matching multiple paths the one that was first `add`ed will be chosen.
//! * This library is in an early stage of development so there may be breaking changes comming
//! (but I'll try as hard as I can not to break backwards compatibility or break it just a little - 
//! I promise I'll try!).
//!
//! # Further Development
//!
//! * add the ability to distinguish requests by query parameters.
//! * maybe some small API changes/upgrades
//! * thinking about returning `Result<Handler, StatusCode>` in case of any error in
//! `find_handler` (and will probably go with it). This will actually enable users to server their
//! own error pages.
//!
//! # Waiting for your feedback
//!
//! I've created this little tool to help myself learn Rust and to avoid using big frameworks
//! like Iron or rustful. I just want to keep things simple.
//!
//! Obviously I could make some errors or bad design choices so I'm waiting for your feedback!
//! You may create an issue at [project's bug tracker](https://github.com/marad/hyper-router/issues).

extern crate hyper;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::server::{Request, Response};

mod path;
pub mod route;
mod builder;
pub mod handlers;

pub use self::path::Path;
pub use self::route::Route;
pub use self::route::RouteBuilder;
pub use self::builder::RouterBuilder;

pub type Handler = fn(Request, Response);

/// This is the one. The router.
#[derive(Debug)]
pub struct Router {
    routes: Vec<Route>
}

impl Router {
    /// Finds handler for given Hyper request.
    ///
    /// If the request does not match any route than default 404 handler is returned.
    /// If the request match some routes but http method does not match (used GET but routes are
    /// defined for POST) than default method not supported handler is returned.
    /// Finally in case of error - internal server error handler is returned.
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

    /// Returns vector of `Route`s that match to given path.
    pub fn find_matching_routes(&self, request_path: &str) -> Vec<&Route> {
        self.routes.iter()
            .filter(|route| {
                route.path.matcher.is_match(&request_path)
            })
            .collect()
    }
}
