use hyper::method::Method;
use std::fmt;

use super::Path;
use super::Handler;

pub struct Route {
    pub method: Method,
    pub path: Path,
    pub handler: Handler
}

impl fmt::Debug for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Route {{method: {:?}, path: {:?}}}", self.method, self.path)
    }
}

