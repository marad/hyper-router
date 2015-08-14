use hyper::method::Method;
use std::fmt;

use super::Path;
use super::Handler;

/// Holds route information
pub struct Route {
    /// HTTP method to match
    pub method: Method,
    /// Path to match
    pub path: Path,
    /// Request handler
    ///
    /// This should be method that accepts Hyper's Request and Response:
    ///
    /// ```rust
    /// fn hello_handler(_: Request, res: Response) {
    ///   res.send(b"Hello World").unwrap();
    /// }
    /// ```
    pub handler: Handler
}

impl fmt::Debug for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Route {{method: {:?}, path: {:?}}}", self.method, self.path)
    }
}
