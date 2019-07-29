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

    fn expected_static_path() -> Path {
        Path::Static("/foo".to_string())
    }

    fn expected_parametric_path() -> Path {
        Path::Parametric(vec!["".to_string(), "foo".to_string(), ":id".to_string()])
    }

    fn some_handler(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    }

    #[test]
    fn test_construct_static_get_route() {
        let r1 = Route::options("/foo", some_handler);
        assert_eq!(r1.method, Method::OPTIONS);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::OPTIONS, "/foo", some_handler);
        assert_eq!(r2.method, Method::OPTIONS);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_static_options_route() {
        let r1 = Route::get("/foo", some_handler);
        assert_eq!(r1.method, Method::GET);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::GET, "/foo", some_handler);
        assert_eq!(r2.method, Method::GET);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_static_post_route() {
        let r1 = Route::post("/foo", some_handler);
        assert_eq!(r1.method, Method::POST);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::POST, "/foo", some_handler);
        assert_eq!(r2.method, Method::POST);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_static_put_route() {
        let r1 = Route::put("/foo", some_handler);
        assert_eq!(r1.method, Method::PUT);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::PUT, "/foo", some_handler);
        assert_eq!(r2.method, Method::PUT);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_static_delete_route() {
        let r1 = Route::delete("/foo", some_handler);
        assert_eq!(r1.method, Method::DELETE);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::DELETE, "/foo", some_handler);
        assert_eq!(r2.method, Method::DELETE);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_static_head_route() {
        let r1 = Route::head("/foo", some_handler);
        assert_eq!(r1.method, Method::HEAD);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::HEAD, "/foo", some_handler);
        assert_eq!(r2.method, Method::HEAD);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_static_trace_route() {
        let r1 = Route::trace("/foo", some_handler);
        assert_eq!(r1.method, Method::TRACE);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::TRACE, "/foo", some_handler);
        assert_eq!(r2.method, Method::TRACE);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_static_connect_route() {
        let r1 = Route::connect("/foo", some_handler);
        assert_eq!(r1.method, Method::CONNECT);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::CONNECT, "/foo", some_handler);
        assert_eq!(r2.method, Method::CONNECT);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_static_patch_route() {
        let r1 = Route::patch("/foo", some_handler);
        assert_eq!(r1.method, Method::PATCH);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::PATCH, "/foo", some_handler);
        assert_eq!(r2.method, Method::PATCH);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_get_route() {
        let r1 = Route::options("/foo/:id", some_handler);
        assert_eq!(r1.method, Method::OPTIONS);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::OPTIONS, "/foo/:id", some_handler);
        assert_eq!(r2.method, Method::OPTIONS);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_options_route() {
        let r1 = Route::get("/foo/:id", some_handler);
        assert_eq!(r1.method, Method::GET);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::GET, "/foo/:id", some_handler);
        assert_eq!(r2.method, Method::GET);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_post_route() {
        let r1 = Route::post("/foo/:id", some_handler);
        assert_eq!(r1.method, Method::POST);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::POST, "/foo/:id", some_handler);
        assert_eq!(r2.method, Method::POST);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_put_route() {
        let r1 = Route::put("/foo/:id", some_handler);
        assert_eq!(r1.method, Method::PUT);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::PUT, "/foo/:id", some_handler);
        assert_eq!(r2.method, Method::PUT);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_delete_route() {
        let r1 = Route::delete("/foo/:id", some_handler);
        assert_eq!(r1.method, Method::DELETE);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::DELETE, "/foo/:id", some_handler);
        assert_eq!(r2.method, Method::DELETE);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_head_route() {
        let r1 = Route::head("/foo/:id", some_handler);
        assert_eq!(r1.method, Method::HEAD);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::HEAD, "/foo/:id", some_handler);
        assert_eq!(r2.method, Method::HEAD);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_trace_route() {
        let r1 = Route::trace("/foo/:id", some_handler);
        assert_eq!(r1.method, Method::TRACE);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::TRACE, "/foo/:id", some_handler);
        assert_eq!(r2.method, Method::TRACE);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_connect_route() {
        let r1 = Route::connect("/foo/:id", some_handler);
        assert_eq!(r1.method, Method::CONNECT);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::CONNECT, "/foo/:id", some_handler);
        assert_eq!(r2.method, Method::CONNECT);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_patch_route() {
        let r1 = Route::patch("/foo/:id", some_handler);
        assert_eq!(r1.method, Method::PATCH);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::PATCH, "/foo/:id", some_handler);
        assert_eq!(r2.method, Method::PATCH);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

}
