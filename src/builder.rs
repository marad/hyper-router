use super::Route;
use super::Router;

/// Builder for a router
///
/// Example usage:
///
#[derive(Debug)]
pub struct RouterBuilder {
    routes: Vec<Route>,
}

impl RouterBuilder {
    pub fn new() -> RouterBuilder {
        RouterBuilder { routes: vec![] }
    }

    /// Adds new `Route` for `Router` that is being built.
    ///
    /// Example:
    ///
    /// ```ignore
    /// use hyper::{Body, Request, Response};
    /// use hyper_router::{Route, RouterBuilder};
    ///
    /// fn some_handler(_: Request<Body>) -> Response<Body> {
    ///   // do something
    /// }
    ///
    /// RouterBuilder::new().add(Route::get(r"/person/\d+").using(some_handler));
    /// ```
    pub fn add(mut self, route: Route) -> RouterBuilder {
        self.routes.push(route);
        self
    }

    pub fn build(self) -> Router {
        Router {
            routes: self.routes,
        }
    }
}
