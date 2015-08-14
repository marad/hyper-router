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
    /// ```rust
    /// builder.add(Route {
    ///   method: Get,
    ///   path: Path::new(r"/person/\d+"),
    ///   handler: some_handler
    /// });
    pub fn add(mut self, route: Route) -> RouterBuilder {
        self.routes.push(route);
        self
    }

    pub fn build(self) -> Router {
        Router { routes: self.routes }
    }
}
