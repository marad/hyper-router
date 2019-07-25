#![doc(html_root_url = "https://marad.github.io/hyper-router/doc/hyper_router")]

use futures::future::FutureResult;
use hyper::service::Service;
use hyper::StatusCode;
use hyper::{Body, Request, Response};

pub mod handlers;
mod parameters;
mod path;
mod route;
mod router;

pub use self::parameters::RouteParameters;
use self::path::Path;
pub use self::route::Route;
pub use self::router::Router;

pub type Handler = fn(Request<Body>) -> Response<Body>;
pub type HttpResult<T> = Result<T, StatusCode>;

/// The default simple router service.
#[derive(Debug)]
pub struct RouterService {
    pub router: Router,
}

impl RouterService {
    pub fn new(router: Router) -> RouterService {
        RouterService { router }
    }
}

impl Service for RouterService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = FutureResult<Response<Body>, hyper::Error>;

    fn call(&mut self, mut request: Request<Self::ReqBody>) -> Self::Future {
        let (handler, parameters) = self.router.find_handler(&request);
        let extensions = request.extensions_mut();
        extensions.insert(parameters);
        futures::future::ok(handler(request))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::spawn;

    use hyper::header::*;
    use hyper::*;
    use std::str::FromStr;

    #[test]
    fn test_router_service_calls_correct_handler_for_static_path() {
        fn get_foo_handler(_: Request<Body>) -> Response<Body> {
            let body = "Hello World";
            Response::builder()
                .header(CONTENT_LENGTH, body.len() as u64)
                .header(CONTENT_TYPE, "text/plain")
                .body(Body::from(body))
                .expect("Failed to construct the response")
        }

        let router = Router::new().add(Route::get("/foo", get_foo_handler));

        let mut svc = RouterService::new(router);

        let request = Request::builder()
            .method(Method::GET)
            .uri(Uri::from_str("http://www.example.com/foo").unwrap())
            .body(Body::empty())
            .unwrap();

        let response = spawn(svc.call(request)).wait_future().unwrap();
        let (parts, body) = response.into_parts();
        let headers = parts.headers;
        assert_eq!(parts.version, Version::HTTP_11);
        assert_eq!(parts.status, 200);
        assert_eq!(headers.get(CONTENT_TYPE).unwrap(), "text/plain");
        assert_eq!(headers.get(CONTENT_LENGTH).unwrap(), "11");

        let bytes = spawn(body).wait_stream().unwrap().unwrap().into_bytes();
        assert_eq!(bytes[..], b"Hello World"[..]);
    }

    #[test]
    fn test_router_service_passes_captured_parameters_to_handler_via_extensions() {
        fn get_foo_handler(req: Request<Body>) -> Response<Body> {
            let route_params: &RouteParameters = req.extensions().get().unwrap();
            let id = route_params.parameters.iter().nth(0).unwrap();
            let body = format!("FOO-{}", id);
            Response::builder()
                .header(CONTENT_LENGTH, body.len() as u64)
                .header(CONTENT_TYPE, "text/plain")
                .body(Body::from(body))
                .expect("Failed to construct the response")
        }

        let router = Router::new().add(Route::get("/foo/:id", get_foo_handler));

        let mut svc = RouterService::new(router);
        let uri = Uri::from_str("http://www.example.com/foo/75bd8cfc-e421-48f8-93a1-57f423d254e6");
        let request = Request::builder()
            .method(Method::GET)
            .uri(uri.unwrap())
            .body(Body::empty())
            .unwrap();

        let response = spawn(svc.call(request)).wait_future().unwrap();
        let (parts, body) = response.into_parts();
        let headers = parts.headers;
        assert_eq!(parts.version, Version::HTTP_11);
        assert_eq!(parts.status, 200);
        assert_eq!(headers.get(CONTENT_TYPE).unwrap(), "text/plain");
        assert_eq!(headers.get(CONTENT_LENGTH).unwrap(), "40");

        let bytes = spawn(body).wait_stream().unwrap().unwrap().into_bytes();
        assert_eq!(bytes[..], b"FOO-75bd8cfc-e421-48f8-93a1-57f423d254e6"[..]);
    }
}
