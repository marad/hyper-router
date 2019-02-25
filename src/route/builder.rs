use crate::Route;
use crate::Handler;

pub struct RouteBuilder {
    route: Route
}

impl RouteBuilder {
    pub fn new(route: Route) -> RouteBuilder {
        RouteBuilder {
            route
        }
    }

    /// Completes the building process by taking the handler to process the request.
    ///
    /// Returns created route.
    pub fn using(mut self, handler: Handler) -> Route {
        self.route.handler = handler;
        self.route
    }
}
