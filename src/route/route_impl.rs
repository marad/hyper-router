use crate::handlers;
use hyper::Method;
use std::fmt;

use super::RouteBuilder;
use crate::Handler;
use crate::Path;

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
    /// ```ignore
    /// use hyper::server::{Request, Response};
    /// use hyper::header::{ContentLength, ContentType};
    ///
    /// fn hello_handler(_: Request) -> Response {
    ///     let body = "Hello World";
    ///     Response::new()
    ///         .with_header(ContentLength(body.len() as u64))
    ///         .with_header(ContentType::plaintext())
    ///         .with_body(body)
    /// }
    /// ```
    pub handler: Handler,
}

impl Route {
    pub fn options(path: &str) -> RouteBuilder {
        Route::from(Method::OPTIONS, path)
    }

    pub fn get(path: &str) -> RouteBuilder {
        Route::from(Method::GET, path)
    }

    pub fn post(path: &str) -> RouteBuilder {
        Route::from(Method::POST, path)
    }

    pub fn put(path: &str) -> RouteBuilder {
        Route::from(Method::PUT, path)
    }

    pub fn delete(path: &str) -> RouteBuilder {
        Route::from(Method::DELETE, path)
    }

    pub fn head(path: &str) -> RouteBuilder {
        Route::from(Method::HEAD, path)
    }

    pub fn trace(path: &str) -> RouteBuilder {
        Route::from(Method::TRACE, path)
    }

    pub fn connect(path: &str) -> RouteBuilder {
        Route::from(Method::CONNECT, path)
    }

    pub fn patch(path: &str) -> RouteBuilder {
        Route::from(Method::PATCH, path)
    }

    pub fn from(method: Method, path: &str) -> RouteBuilder {
        RouteBuilder::new(Route {
            method,
            path: Path::new(path),
            ..Route::default()
        })
    }
}

impl Default for Route {
    fn default() -> Route {
        Route {
            method: Method::GET,
            path: Path::new("/"),
            handler: handlers::not_implemented_handler,
        }
    }
}

impl fmt::Debug for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Route {{method: {:?}, path: {:?}}}",
            self.method, self.path
        )
    }
}
