use crate::handlers;
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

pub struct RouteBuilder {
    route: Route,
}

impl RouteBuilder {
    pub fn new(route: Route) -> RouteBuilder {
        RouteBuilder { route }
    }

    /// Completes the building process by taking the handler to process the request.
    ///
    /// Returns created route.
    pub fn using(mut self, handler: Handler) -> Route {
        self.route.handler = handler;
        self.route
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
        let r1 = Route::options("/foo").using(some_handler);
        assert_eq!(r1.method, Method::OPTIONS);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::OPTIONS, "/foo").using(some_handler);
        assert_eq!(r2.method, Method::OPTIONS);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_static_options_route() {
        let r1 = Route::get("/foo").using(some_handler);
        assert_eq!(r1.method, Method::GET);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::GET, "/foo").using(some_handler);
        assert_eq!(r2.method, Method::GET);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_static_post_route() {
        let r1 = Route::post("/foo").using(some_handler);
        assert_eq!(r1.method, Method::POST);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::POST, "/foo").using(some_handler);
        assert_eq!(r2.method, Method::POST);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_static_put_route() {
        let r1 = Route::put("/foo").using(some_handler);
        assert_eq!(r1.method, Method::PUT);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::PUT, "/foo").using(some_handler);
        assert_eq!(r2.method, Method::PUT);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_static_delete_route() {
        let r1 = Route::delete("/foo").using(some_handler);
        assert_eq!(r1.method, Method::DELETE);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::DELETE, "/foo").using(some_handler);
        assert_eq!(r2.method, Method::DELETE);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_static_head_route() {
        let r1 = Route::head("/foo").using(some_handler);
        assert_eq!(r1.method, Method::HEAD);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::HEAD, "/foo").using(some_handler);
        assert_eq!(r2.method, Method::HEAD);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_static_trace_route() {
        let r1 = Route::trace("/foo").using(some_handler);
        assert_eq!(r1.method, Method::TRACE);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::TRACE, "/foo").using(some_handler);
        assert_eq!(r2.method, Method::TRACE);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_static_connect_route() {
        let r1 = Route::connect("/foo").using(some_handler);
        assert_eq!(r1.method, Method::CONNECT);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::CONNECT, "/foo").using(some_handler);
        assert_eq!(r2.method, Method::CONNECT);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_static_patch_route() {
        let r1 = Route::patch("/foo").using(some_handler);
        assert_eq!(r1.method, Method::PATCH);
        assert_eq!(r1.path, expected_static_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::PATCH, "/foo").using(some_handler);
        assert_eq!(r2.method, Method::PATCH);
        assert_eq!(r2.path, expected_static_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_get_route() {
        let r1 = Route::options("/foo/:id").using(some_handler);
        assert_eq!(r1.method, Method::OPTIONS);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::OPTIONS, "/foo/:id").using(some_handler);
        assert_eq!(r2.method, Method::OPTIONS);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_options_route() {
        let r1 = Route::get("/foo/:id").using(some_handler);
        assert_eq!(r1.method, Method::GET);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::GET, "/foo/:id").using(some_handler);
        assert_eq!(r2.method, Method::GET);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_post_route() {
        let r1 = Route::post("/foo/:id").using(some_handler);
        assert_eq!(r1.method, Method::POST);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::POST, "/foo/:id").using(some_handler);
        assert_eq!(r2.method, Method::POST);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_put_route() {
        let r1 = Route::put("/foo/:id").using(some_handler);
        assert_eq!(r1.method, Method::PUT);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::PUT, "/foo/:id").using(some_handler);
        assert_eq!(r2.method, Method::PUT);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_delete_route() {
        let r1 = Route::delete("/foo/:id").using(some_handler);
        assert_eq!(r1.method, Method::DELETE);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::DELETE, "/foo/:id").using(some_handler);
        assert_eq!(r2.method, Method::DELETE);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_head_route() {
        let r1 = Route::head("/foo/:id").using(some_handler);
        assert_eq!(r1.method, Method::HEAD);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::HEAD, "/foo/:id").using(some_handler);
        assert_eq!(r2.method, Method::HEAD);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_trace_route() {
        let r1 = Route::trace("/foo/:id").using(some_handler);
        assert_eq!(r1.method, Method::TRACE);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::TRACE, "/foo/:id").using(some_handler);
        assert_eq!(r2.method, Method::TRACE);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_connect_route() {
        let r1 = Route::connect("/foo/:id").using(some_handler);
        assert_eq!(r1.method, Method::CONNECT);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::CONNECT, "/foo/:id").using(some_handler);
        assert_eq!(r2.method, Method::CONNECT);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

    #[test]
    fn test_construct_parametric_patch_route() {
        let r1 = Route::patch("/foo/:id").using(some_handler);
        assert_eq!(r1.method, Method::PATCH);
        assert_eq!(r1.path, expected_parametric_path());
        assert_eq!(r1.handler as fn(_) -> _, some_handler as fn(_) -> _);
        let r2 = Route::from(Method::PATCH, "/foo/:id").using(some_handler);
        assert_eq!(r2.method, Method::PATCH);
        assert_eq!(r2.path, expected_parametric_path());
        assert_eq!(r2.handler as fn(_) -> _, some_handler as fn(_) -> _);
    }

}
