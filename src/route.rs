use crate::Handler;
use crate::Path;
use hyper::Method;
use std::fmt;

/// Holds route information
pub struct Route {
    /// HTTP method to match
    pub method: Method,

    /// Path to match
    pub(crate) path: Path,

    /// Request handler
    ///
    /// This should be method that accepts Hyper's Request and Response:
    ///
    /// ```rust
    /// use hyper::*;
    /// use hyper::header::*;
    ///
    /// fn hello_handler(_: Request<Body>) -> Response<Body> {
    ///     let body = "Hello World";
    ///     Response::builder()
    ///          .header(CONTENT_LENGTH, body.len() as u64)
    ///          .header(CONTENT_TYPE, "text/plain")
    ///          .body(Body::from(body))
    ///          .expect("Failed to construct the response")
    /// }
    /// ```
    pub handler: Handler,
}

impl Route {
    pub fn options(path: &str, handler: Handler) -> Route {
        Route::from(Method::OPTIONS, path, handler)
    }

    pub fn get(path: &str, handler: Handler) -> Route {
        Route::from(Method::GET, path, handler)
    }

    pub fn post(path: &str, handler: Handler) -> Route {
        Route::from(Method::POST, path, handler)
    }

    pub fn put(path: &str, handler: Handler) -> Route {
        Route::from(Method::PUT, path, handler)
    }

    pub fn delete(path: &str, handler: Handler) -> Route {
        Route::from(Method::DELETE, path, handler)
    }

    pub fn head(path: &str, handler: Handler) -> Route {
        Route::from(Method::HEAD, path, handler)
    }

    pub fn trace(path: &str, handler: Handler) -> Route {
        Route::from(Method::TRACE, path, handler)
    }

    pub fn connect(path: &str, handler: Handler) -> Route {
        Route::from(Method::CONNECT, path, handler)
    }

    pub fn patch(path: &str, handler: Handler) -> Route {
        Route::from(Method::PATCH, path, handler)
    }

    pub fn from(method: Method, path: &str, handler: Handler) -> Route {
        Route {
            method,
            path: Path::new(path),
            handler: handler,
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

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::*;

    fn some_handler(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    }

    #[test]
    fn test_construct_get_route() {
        let r = Route::get("/foo", some_handler);
        assert_eq!(r.method, Method::GET);
    }
}
