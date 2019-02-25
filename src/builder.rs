use super::Route;
use super::Router;
use std;

/// Builder for a router
///
/// Example usage:
///
#[derive(Debug, Default)]
pub struct RouterBuilder {
    routes: Vec<Route>,
}

impl RouterBuilder {
    pub fn new() -> RouterBuilder {
        RouterBuilder { routes: vec![] }
    }

    pub fn build(self) -> Router {
        Router {
            routes: self.routes,
        }
    }
}

impl std::ops::Add<crate::route::Route> for RouterBuilder {
    type Output = RouterBuilder;

    /// Adds new `Route` for `Router` that is being built.
    ///
    /// Example:
    ///
    /// ```ignore
    /// use hyper::server::{Request, Response};
    /// use hyper_router::{Route, RouterBuilder};
    ///
    /// fn some_handler(_: Request) -> Response {
    ///   // do something
    /// }
    ///
    /// RouterBuilder::new().add(Route::get(r"/person/\d+").using(some_handler));
    /// ```
    fn add(mut self, route: Route) -> RouterBuilder {
        self.routes.push(route);
        self
    }
}
