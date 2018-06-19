use hyper::Body;
use hyper::{Request, Response};
use hyper::StatusCode;
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use http::Result;

pub fn default_404_handler(_: Request<Body>) -> Response<Body> {
    let body = "page not found";
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct \"Not Found\" response")
}

pub fn method_not_supported_handler(_: Request<Body>) -> Response<Body> {
    let body = "method not supported";
    Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct \"Method Not Supported\" response")
}

pub fn internal_server_error_handler(_: Request<Body>) -> Response<Body> {
    let body = "internal server error";
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct \"Internal Server Error\" response")
}

pub fn not_implemented_handler(_: Request<Body>) -> Response<Body> {
    let body = "not implemented";
    Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct \"Not Implemented\" response")
}
