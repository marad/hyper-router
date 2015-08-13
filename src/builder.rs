use super::Route;
use super::Router;

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
